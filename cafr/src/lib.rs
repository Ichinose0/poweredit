pub mod widget;

use std::fmt::Debug;

use widget::Target;

pub trait Application: Sized {
    type Message: Send + Debug;

    fn ui(&mut self) -> Target<Self::Message>;
}

pub fn run<T>(mut app: T) 
where
    T: Application,
{
}