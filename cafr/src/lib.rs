pub mod widget;

use std::fmt::Debug;

use widget::Target;

pub struct Executable {

}

impl Executable {
    pub fn new() -> Self {
        Self {
            
        }
    }

    pub fn run<T>(self,mut app: T) 
    where
        T: Application,
    {

    }
}

pub trait Application: Sized {
    type Message: Send + Debug;

    fn ui(&mut self) -> Target<Self::Message>;
}

