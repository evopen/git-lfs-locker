use crate::action;
use crate::storage::Storage;
use anyhow::Result;
use std::path::{Path, PathBuf};
use std::rc::{Rc, Weak};

pub struct Ui {
    ui_instance: egui_winit_platform::Platform,
    ui_render_pass: egui_wgpu_backend::RenderPass,
    screen_desc: egui_wgpu_backend::ScreenDescriptor,
    device: Weak<wgpu::Device>,
    queue: Weak<wgpu::Queue>,
    paint_jobs: egui::PaintJobs,
    storage: Storage,
}

impl Ui {
    pub fn new(
        platform_desc: egui_winit_platform::PlatformDescriptor,
        device: Weak<wgpu::Device>,
        queue: Weak<wgpu::Queue>,
        format: wgpu::TextureFormat,
    ) -> Self {
        let screen_descriptor = egui_wgpu_backend::ScreenDescriptor {
            physical_width: platform_desc.physical_width,
            physical_height: platform_desc.physical_height,
            scale_factor: platform_desc.scale_factor as f32,
        };
        let ui_instance = egui_winit_platform::Platform::new(platform_desc);
        let ui_render_pass =
            egui_wgpu_backend::RenderPass::new(device.upgrade().unwrap().as_ref(), format);

        let storage = Storage::default();

        Self {
            ui_instance,
            ui_render_pass,
            screen_desc: screen_descriptor,
            device,
            queue,
            paint_jobs: vec![],
            storage,
        }
    }

    pub fn draw(&mut self) {
        self.ui_instance.begin_frame();
        egui::TopPanel::top(egui::Id::new("menu bar")).show(&self.ui_instance.context(), |ui| {
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Open").clicked {
                        match action::pick_repo() {
                            Ok(p) => {
                                if let Some(p) = p {
                                    self.storage.repo_path = p;
                                    self.storage.message.clear();
                                    log::info!(
                                        "picked {}",
                                        self.storage.repo_path.to_str().unwrap()
                                    );
                                }
                            }
                            Err(s) => {
                                self.storage.message = s;
                                log::warn!("{}", self.storage.message);
                            }
                        }
                    }
                });
                ui.with_layout(
                    egui::Layout::horizontal(egui::Align::Center).reverse(),
                    |ui| {
                        ui.label(format!(
                            "Current Repo: {}",
                            &self.storage.repo_path.to_str().unwrap()
                        ));
                    },
                );
            })
        });
        egui::CentralPanel::default().show(&self.ui_instance.context(), |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.add(egui::TextEdit::new(&mut self.storage.filter_text).multiline(false));
                    ui.with_layout(
                        egui::Layout::horizontal(egui::Align::Center).reverse(),
                        |ui| {
                            ui.button("unlock all");
                        },
                    );
                });
                egui::ScrollArea::from_max_height(std::f32::INFINITY).show(ui, |ui| {
                    ui.left_column(200.0).add(egui::Label::new("File"));
                    ui.centered_column(200.0).add(egui::Label::new("Locked By"));
                    ui.right_column(200.0).add(egui::Label::new("Action"));
                });
            });
        });
        let (_, paint_cmds) = self.ui_instance.end_frame();
        self.paint_jobs = self.ui_instance.context().tesselate(paint_cmds);
    }

    pub fn input<T>(&mut self, event: &winit::event::Event<T>, time: f64) {
        self.ui_instance.update_time(time);
        self.ui_instance.handle_event(event);
    }

    pub fn update(&mut self) {
        let device = self.device.upgrade().unwrap();
        let queue = self.queue.upgrade().unwrap();

        self.ui_render_pass
            .update_buffers(&device, &queue, &self.paint_jobs, &self.screen_desc);
        self.ui_render_pass
            .update_texture(&device, &queue, &self.ui_instance.context().texture());
    }

    pub fn encode(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        color: Option<wgpu::Color>,
    ) {
        self.ui_render_pass
            .execute(encoder, view, &self.paint_jobs, &self.screen_desc, color);
    }
}
