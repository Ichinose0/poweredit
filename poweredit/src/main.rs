use cafr::{Application, Executable, widget::Container, frame::Frame};

#[derive(Debug)]
pub enum Message {
    
}

pub struct Poweredit {
    frame: MyFrame
}

impl Application for Poweredit {
    type Message = Message;

    fn ui(&mut self) -> cafr::widget::Target<Self::Message> {
        Container::new().into()
    }

    fn init(&mut self,loader: &cafr::plugin::PluginLoader) {
        
    }

    fn route(&mut self) -> &dyn cafr::frame::Frame {
        &self.frame
    }
}

pub struct MyFrame {}

impl Frame for MyFrame {
    type Message = Message;

    fn bgr(&self) -> cafr::Color {
        cafr::Color::ARGB(255,125,0,255)
    }

    fn title(&self) -> String {
        "サンプルフレーム".to_owned()
    }

    fn ui(&mut self) -> cafr::widget::Target<Self::Message> {
        todo!()
    }
}

fn main() {
    let exe = Executable::new();
    let app = Poweredit { frame: MyFrame {} };
    exe.run(app);
}
