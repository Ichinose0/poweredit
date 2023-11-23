use crate::Color;

use super::{Element, Widget, WidgetType, Text, Target};

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

    pub fn element(self) -> Element<T> {
        Element {
            widget: Box::new(self.inner),
            msg: None
        }
    }

    pub fn build(self) -> Target<T> {
        let text = Text::new()
                        .text(self.inner.text.clone());
                    
        Target {
            inner: vec![
                Element {
                    widget: Box::new(self.inner),
                    msg: None
                },
                text.element(),
            ]
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

    fn widget_type(&self) -> WidgetType {
        WidgetType::Rectangle
    }

    fn title(&self) -> &str {
        &self.text
    }
}