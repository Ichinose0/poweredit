use cafr::{Application, Executable, frame::Frame, ApplicationEvent};

#[derive(Debug)]
pub enum Message {
    
}

pub struct Poweredit {
    frame: MyFrame
}

impl Application for Poweredit {
    type Message = Message;

    fn init(&mut self,loader: &cafr::plugin::PluginLoader) {
        
    }

    fn route(&mut self,event: ApplicationEvent) -> &dyn Frame<Message = Self::Message> {
        &self.frame
    }

    fn on_close(&mut self) {

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

    fn ui(&self) -> cafr::widget::Target<Self::Message> {
        let text = cafr::widget::Button::new()
                                            .width(240)
                                            .height(80)
                                            .text(String::from("ボタン"));

        text.build()
    }

    fn resizable(&self) -> bool {
        true
    }
}

fn main() {
    let exe = Executable::new();
    let app = Poweredit { frame: MyFrame {} };
    exe.run(app);
}
