use crate::Color;

pub struct Target<T> {
    pub(crate)widget: Box<dyn Widget>,
    pub(crate)msg: Option<T>
}

impl<T> Target<T> {
    pub fn get(&self) -> Vec<&Box<dyn Widget>> {
        vec![&self.widget]
    }
}

pub trait Widget {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn color(&self) -> Color;
    fn shadow(&self) -> Shadow {
        Shadow {
            color: Color::ARGB(255,128,128,128),
            border: 1
        }
    }
}

pub struct Shadow {
    pub(crate) color: Color,
    pub(crate) border: u32
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

    pub fn build(self) -> Target<T> {
        Target {
            widget: Box::new(_Container {}),
            msg: None,
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

    fn color(&self) -> Color {
        Color::White
    }
}

pub struct Button<T> 
where
    T: Send + std::fmt::Debug
{
    inner: _Button<T>
}

impl<T> Button<T>
where
    T: Send + std::fmt::Debug + 'static
{
    pub fn new() -> Self {
        Button::default()
    }

    pub fn text(mut self,text: String) -> Self {
        self.inner.text = text;
        self
    }

    pub fn width(mut self,width: u32) -> Self {
        self.inner.width = width;
        self
    }

    pub fn height(mut self,height: u32) -> Self {
        self.inner.height = height;
        self
    }

    pub fn build(self) -> Target<T> {
        Target {
            widget: Box::new(self.inner),
            msg: None
        }
    }
}

impl<T> Default for Button<T> 
where
    T: Send + std::fmt::Debug
{
    fn default() -> Self {
        Self {
            inner: _Button { on_click: Default::default(), text: "".to_owned(), width: 120, height: 40 },
        }
    }
}

struct _Button<T> 
where
    T: Send + std::fmt::Debug 
{
    text: String,
    width: u32,
    height: u32,
    on_click: Option<T>
}

impl<T> Widget for _Button<T> 
where
    T: Send + std::fmt::Debug
{
    fn width(&self) -> u32 {
        self.width
    }

    fn color(&self) -> Color {
        Color::White
    }

    fn height(&self) -> u32 {
        self.height
    }
}