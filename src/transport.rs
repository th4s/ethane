use std::error::Error;

pub mod ws;

pub trait Request<T: Error> {
    fn request(&mut self, cmd: String) -> Result<String, T>;
}

// pub trait Subscribe {
//     fn subscribe<'a, T>(&mut self) -> Receiver<T>
//     where
//         T: Deserialize<'a>;
// }
