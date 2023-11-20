use cafr::{Application, Executable, widget::Container};

#[derive(Debug)]
pub enum Message {

}

pub struct Poweredit {

}

impl Application for Poweredit {
    type Message = Message;

    fn ui(&mut self) -> cafr::widget::Target<Self::Message> {
        Container::new().into()
    }

    fn init(&mut self,loader: &cafr::plugin::PluginLoader) {
        
    }
}

fn main() {
    let exe = Executable::new();
    let app = Poweredit {};
    exe.run(app);
}
