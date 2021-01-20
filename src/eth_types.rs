use ethereum_types::{Address, Bloom, H256, U256, U64};
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransactionRequest {
    pub from: Address,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas: Option<U256>,
    #[serde(rename = "gasPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<U256>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<U256>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Bytes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<U256>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    #[serde(rename = "blockHash")]
    pub block_hash: Option<H256>,
    #[serde(rename = "blockNumber")]
    pub block_number: Option<U64>,
    pub from: Address,
    pub gas: U256,
    #[serde(rename = "gasPrice")]
    pub gas_price: U256,
    pub hash: H256,
    pub input: Bytes,
    pub nonce: U256,
    pub to: Option<Address>,
    #[serde(rename = "transactionIndex")]
    pub transaction_index: Option<U64>,
    pub value: U256,
    pub v: U64,
    pub r: U256,
    pub s: U256,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionReceipt {
    #[serde(rename = "transactionHash")]
    pub transaction_hash: H256,
    #[serde(rename = "transactionIndex")]
    pub transaction_index: U64,
    #[serde(rename = "blockHash")]
    pub block_hash: H256,
    #[serde(rename = "blockNumber")]
    pub block_number: U64,
    pub from: Address,
    pub to: Option<Address>,
    #[serde(rename = "cumulativeGasUsed")]
    pub cumulative_gas_used: U256,
    #[serde(rename = "gasUsed")]
    pub gas_used: U256,
    #[serde(rename = "contractAddress")]
    pub contract_address: Option<Address>,
    pub logs: Vec<Log>,
    #[serde(rename = "logsBloom")]
    pub logs_bloom: Bloom,
    pub status: U64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Log {
    address: Address,
    topics: Vec<H256>,
    data: Bytes,
    #[serde(rename = "blockHash")]
    pub block_hash: Option<H256>,
    #[serde(rename = "blockNumber")]
    pub block_number: Option<U64>,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: Option<H256>,
    #[serde(rename = "transactionIndex")]
    pub transaction_index: Option<U64>,
    #[serde(rename = "logIndex")]
    pub log_index: Option<U256>,
    #[serde(rename = "transactionLogIndex")]
    pub transaction_log_index: Option<U256>,
    pub removed: bool,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Bytes(pub Vec<u8>);

impl Serialize for Bytes {
    fn serialize<T: Serializer>(&self, serializer: T) -> Result<T::Ok, T::Error> {
        serializer.serialize_str(&(String::from("0x") + &hex::encode(&self.0)))
    }
}

impl<'de> Deserialize<'de> for Bytes {
    fn deserialize<T>(deserializer: T) -> Result<Bytes, T::Error>
    where
        T: Deserializer<'de>,
    {
        deserializer.deserialize_identifier(BytesVisitor)
    }
}

struct BytesVisitor;

impl<'de> Visitor<'de> for BytesVisitor {
    type Value = Bytes;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a hex string")
    }

    fn visit_str<T: serde::de::Error>(self, value: &str) -> Result<Self::Value, T> {
        let inner = if value.len() >= 2 && &value[0..2] == "0x" {
            hex::decode(&value[2..])
        } else {
            hex::decode(value)
        }
        .map_err(|err| serde::de::Error::custom(format!("Invalid hex string: {}", err)))?;

        Ok(Bytes(inner))
    }

    fn visit_string<T: serde::de::Error>(self, value: String) -> Result<Self::Value, T> {
        self.visit_str(&value)
    }
}
