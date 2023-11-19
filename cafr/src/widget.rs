pub struct Target<T> 
where
    T: Send + std::fmt::Debug
{
    msg: Option<T>,
    widget: Box<dyn Widget>
}

pub trait Widget {
    fn width(&self) -> u32;
    fn height(&self) -> u32;

}

pub struct Container<T> 
where
    T: Send + std::fmt::Debug
{
    on_click: Option<T>
}

impl<T> Container<T>
where
    T: Send + std::fmt::Debug
{
    pub fn new() -> Self {
        Self {
            on_click: None
        }
    }

    pub fn into(self) -> Target<T> {
        Target {
            msg: self.on_click,
            widget: Box::new(_Container {})
        }
    }
}

pub struct _Container {}

impl Widget for _Container {
    fn width(&self) -> u32 {
        30
    }

    fn height(&self) -> u32 {
        30
    }
}