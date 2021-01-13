use std::error::Error;

pub mod ws;

pub trait Request {
    fn request(&mut self, cmd: String) -> Result<String, Box<dyn Error>>;
}

// pub trait Subscribe {
//     fn subscribe(&mut self, cmd: String) -> Result<Receiver<String>, Box<dyn Error>>;
// }
