use std::sync::Arc;

use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

pub struct Window {
    pub raw_window: Arc<winit::window::Window>,
    pub event_loop: EventLoop<()>,
}

impl Window {
    pub fn new() -> Self {
        let event_loop = EventLoop::new(); // ignore this for now
        let window = Arc::new(WindowBuilder::new().build(&event_loop).unwrap());
        Window {
            raw_window: window,
            event_loop: event_loop,
        }
    }

    pub fn run(self) {
        self.event_loop.run(|event, _, control_flow| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => (),
        });
    }
}
