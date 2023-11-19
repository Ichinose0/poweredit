pub mod widget;

use std::fmt::Debug;

use widget::Target;

use winit::{
    window::Window,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

pub struct Executable {
    window: Window,
    event_loop: EventLoop<()>
}

impl Executable {
    pub fn new() -> Self {
        let event_loop = EventLoop::new().unwrap();

        let window = WindowBuilder::new()
            .with_title("A fantastic window!")
            .with_inner_size(winit::dpi::LogicalSize::new(128.0, 128.0))
            .build(&event_loop)
            .unwrap();
        Self {
            window,
            event_loop
        }
    }

    pub fn run<T>(self,mut app: T) 
    where
        T: Application,
    {
        self.event_loop.run(move |event, elwt| {
            println!("{event:?}");
    
            match event {
                Event::WindowEvent { event, window_id } if window_id == self.window.id() => match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::RedrawRequested => {
                        app.ui();
                        // Notify the windowing system that we'll be presenting to the window.
                        self.window.pre_present_notify();
                    }
                    _ => (),
                },
                Event::AboutToWait => {
                    self.window.request_redraw();
                }
    
                _ => (),
            }
        });
    }
}

pub trait Application: Sized {
    type Message: Send + Debug;

    fn ui(&mut self) -> Target<Self::Message>;
}

