use crossbeam_channel::Receiver;
use serde::Deserialize;

pub mod ws;

pub trait Request {
    fn request<'a, T, U>(&mut self) -> T
    where
        T: Deserialize<'a>;
}

pub trait Subscribe {
    fn subscribe<'a, T>(&mut self) -> Receiver<T>
    where
        T: Deserialize<'a>;
}
