use ethereum_types::U64;
use serde::{Serialize, Serializer};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BlockParameter {
    Latest,
    Earliest,
    Pending,
    Custom(U64),
}

impl Serialize for BlockParameter {
    fn serialize<T: Serializer>(&self, serializer: T) -> Result<T::Ok, T::Error> {
        match *self {
            BlockParameter::Latest => serializer.serialize_str("latest"),
            BlockParameter::Earliest => serializer.serialize_str("earliest"),
            BlockParameter::Pending => serializer.serialize_str("pending"),
            BlockParameter::Custom(num) => serializer.serialize_str(&format!("{:#x}", num)),
        }
    }
}
