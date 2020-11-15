use crate::ui::Ui;
use std::rc::Rc;

pub struct Engine {
    size: winit::dpi::PhysicalSize<u32>,
    instance: wgpu::Instance,
    device: Rc<wgpu::Device>,
    queue: Rc<wgpu::Queue>,
    swap_chain_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    ui: Ui,
    start_instant: std::time::Instant,
}

impl Engine {
    pub async fn new(window: &winit::window::Window) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::BackendBit::VULKAN);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    shader_validation: true,
                },
                None,
            )
            .await
            .unwrap()
            .into();
        let device = Rc::new(device);
        let queue = Rc::new(queue);

        let swap_chain_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        let swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);

        let ui = Ui::new(
            egui_winit_platform::PlatformDescriptor {
                physical_width: size.width,
                physical_height: size.height,
                scale_factor: window.scale_factor(),
                font_definitions: egui::FontDefinitions::with_pixels_per_point(
                    window.scale_factor() as f32,
                ),
                style: egui::Style::default(),
            },
            Rc::downgrade(&device),
            Rc::downgrade(&queue),
            swap_chain_desc.format,
        );

        let start_instant = std::time::Instant::now();

        Self {
            instance,
            device,
            queue,
            size,
            swap_chain_desc,
            swap_chain,
            ui,
            start_instant,
        }
    }

    pub fn update(&mut self) {
        self.ui.draw();
        self.ui.update();
    }

    pub fn input<T>(&mut self, event: &winit::event::Event<T>) {
        self.ui
            .input(event, self.start_instant.elapsed().as_secs_f64());
    }

    pub fn render(&mut self) {
        let frame = self.swap_chain.get_current_frame().unwrap().output;
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("main encoder"),
            });

        self.ui
            .encode(&mut encoder, &frame.view, Some(wgpu::Color::BLACK));

        self.queue.submit(std::iter::once(encoder.finish()));
    }
}
