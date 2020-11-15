mod action;
mod engine;
mod storage;
mod ui;

fn main() {
    env_logger::init();
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title(env!("CARGO_PKG_NAME"))
        .with_inner_size(winit::dpi::LogicalSize::new(640, 480))
        .with_resizable(true)
        .with_transparent(false)
        .build(&event_loop)
        .unwrap();

    let mut engine = futures::executor::block_on(engine::Engine::new(&window));

    event_loop.run(move |winit_event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Wait;
        match winit_event {
            winit::event::Event::WindowEvent { ref event, .. } => {
                engine.input(&winit_event);
                window.request_redraw();
                match event {
                    winit::event::WindowEvent::CloseRequested => {
                        *control_flow = winit::event_loop::ControlFlow::Exit
                    }
                    _ => {}
                }
            }
            winit::event::Event::RedrawRequested(_) => {
                engine.update();
                engine.render();
            }
            _ => {}
        }
    });
}
