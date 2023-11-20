use crate::{Color, widget::Target};

pub trait Frame: Sized {
    type Message: Send + std::fmt::Debug;

    fn bgr(&self) -> Color;
    fn title(&self) -> String;
    fn ui(&mut self) -> Target<Self::Message>;
}