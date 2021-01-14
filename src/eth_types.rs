use ethereum_types::{Address, H256};
use hex::ToHex;
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use std::fmt::{Display, Formatter, Result as FmtResult};

pub enum BlockParameter {
    LATEST,
    EARLIEST,
    PENDING,
    CUSTOM(u32),
}

impl Display for BlockParameter {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let block_param = match *self {
            BlockParameter::LATEST => String::from("latest"),
            BlockParameter::EARLIEST => String::from("earliest"),
            BlockParameter::PENDING => String::from("pending"),
            BlockParameter::CUSTOM(num) => format!("{:#x}", num),
        };
        write!(f, "{}", block_param)
    }
}

pub struct HexBytes(Vec<u8>);

impl Display for HexBytes {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let inner_hex: String = self.0.encode_hex();
        let hex = String::from("0x") + &inner_hex;
        write!(f, "{}", hex)
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct HexNumber(#[serde(deserialize_with = "from_hex_number")] pub u32);

fn from_hex_number<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    u32::from_str_radix(&s[2..], 16)
        .map_err(|_| D::Error::custom("Unable to deserialize hex number"))
}

pub struct Transaction {
    pub from: Address,
    pub to: Address,
    pub gas: u32,
    pub gas_price: u32,
    pub value: u32,
    pub data: HexBytes,
    pub nonce: u32,
}

impl Display for Transaction {
    fn fmt(&self, _f: &mut Formatter<'_>) -> FmtResult {
        todo!()
    }
}

pub struct Filter {
    pub from_block: BlockParameter,
    pub to_block: BlockParameter,
    pub address: Address,
    pub topics: Vec<H256>,
}

impl Display for Filter {
    fn fmt(&self, _f: &mut Formatter<'_>) -> FmtResult {
        todo!()
    }
}

pub struct Filter234 {
    pub from_block: BlockParameter,
    pub to_block: BlockParameter,
    pub address: Address,
    pub topics: Vec<H256>,
    pub block_hash: H256,
}

impl Display for Filter234 {
    fn fmt(&self, _f: &mut Formatter<'_>) -> FmtResult {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_for_block_parameter() {
        assert_eq!(BlockParameter::CUSTOM(0).to_string(), "0x0");
        assert_eq!(BlockParameter::CUSTOM(17).to_string(), "0x11");
        assert_eq!(BlockParameter::CUSTOM(256).to_string(), "0x100");
    }

    #[test]
    fn test_display_for_bytes() {
        let bytes = HexBytes(vec![0, 1, 122, 4]);
        assert_eq!("0x00017a04", bytes.to_string());
    }
}
