pub use ethereum_types::{Bloom, H160, H256, H64, U128, U256, U64};
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq, Deserialize)]
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

impl Default for BlockParameter {
    fn default() -> Self {
        BlockParameter::Latest
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Default)]
pub struct TransactionRequest {
    pub from: H160,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<H160>,
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

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Transaction {
    #[serde(rename = "blockHash")]
    pub block_hash: Option<H256>,
    #[serde(rename = "blockNumber")]
    pub block_number: Option<U64>,
    pub from: H160,
    pub gas: U256,
    #[serde(rename = "gasPrice")]
    pub gas_price: U256,
    pub hash: H256,
    pub input: Bytes,
    pub nonce: U256,
    pub to: Option<H160>,
    #[serde(rename = "transactionIndex")]
    pub transaction_index: Option<U64>,
    pub value: U256,
    pub v: Option<U64>,
    pub r: Option<U256>,
    pub s: Option<U256>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct TransactionReceipt {
    #[serde(rename = "transactionHash")]
    pub transaction_hash: H256,
    #[serde(rename = "transactionIndex")]
    pub transaction_index: U64,
    #[serde(rename = "blockHash")]
    pub block_hash: H256,
    #[serde(rename = "blockNumber")]
    pub block_number: U64,
    pub from: H160,
    pub to: Option<H160>,
    #[serde(rename = "cumulativeGasUsed")]
    pub cumulative_gas_used: U256,
    #[serde(rename = "gasUsed")]
    pub gas_used: U256,
    #[serde(rename = "contractAddress")]
    pub contract_address: Option<H160>,
    pub logs: Vec<Log>,
    #[serde(rename = "logsBloom")]
    pub logs_bloom: Bloom,
    pub status: U64,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Log {
    pub address: H160,
    pub topics: Vec<H256>,
    pub data: Bytes,
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

impl Bytes {
    pub fn from_slice(slice: &[u8]) -> Self {
        Bytes(slice.to_vec())
    }
}

impl Serialize for Bytes {
    fn serialize<T: Serializer>(&self, serializer: T) -> Result<T::Ok, T::Error> {
        serializer.serialize_str(&(String::from("0x") + &hex::encode(&self.0)))
    }
}

impl FromStr for Bytes {
    type Err = hex::FromHexError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let inner = if value.len() >= 2 && &value[0..2] == "0x" {
            hex::decode(&value[2..])
        } else {
            hex::decode(value)
        }?;

        Ok(Bytes(inner))
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
        let result = Self::Value::from_str(value)
            .map_err(|err| serde::de::Error::custom(format!("Invalid hex string: {}", err)))?;
        Ok(result)
    }

    fn visit_string<T: serde::de::Error>(self, value: String) -> Result<Self::Value, T> {
        self.visit_str(&value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PrivateKey {
    ZeroXPrefixed(H256),
    NonPrefixed(H256),
}

impl Serialize for PrivateKey {
    fn serialize<T: Serializer>(&self, serializer: T) -> Result<T::Ok, T::Error> {
        match *self {
            PrivateKey::ZeroXPrefixed(pk) => pk.serialize(serializer),
            PrivateKey::NonPrefixed(pk) => serializer.serialize_str(&hex::encode(pk.0)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Block {
    pub number: Option<U64>,
    pub hash: Option<H256>,
    #[serde(rename = "parentHash")]
    pub parent_hash: H256,
    pub nonce: Option<H64>,
    #[serde(rename = "sha3Uncles")]
    pub sha3_uncles: H256,
    #[serde(rename = "logsBloom")]
    pub logs_bloom: Option<Bloom>,
    #[serde(rename = "transactionsRoot")]
    pub transactions_root: H256,
    #[serde(rename = "stateRoot")]
    pub state_root: H256,
    #[serde(rename = "receiptsRoot")]
    pub receipts_root: H256,
    pub miner: H160,
    pub difficulty: U256,
    #[serde(rename = "totalDifficulty")]
    pub total_difficulty: U256,
    #[serde(rename = "extraData")]
    pub extra_data: Bytes,
    pub size: U256,
    #[serde(rename = "gasLimit")]
    pub gas_limit: U256,
    #[serde(rename = "gasUsed")]
    pub gas_used: U256,
    pub timestamp: U256,
    pub transactions: Vec<TransactionOrHash>,
    pub uncles: Vec<H256>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum TransactionOrHash {
    Transaction(Transaction),
    Hash(H256),
}

#[derive(Clone, Debug, PartialEq, Serialize, Default)]
pub struct Call {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<H160>,
    pub to: H160,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas: Option<U256>,
    #[serde(rename = "gasPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<U256>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<U256>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Bytes>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Default)]
pub struct GasCall {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<H160>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<H160>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas: Option<U256>,
    #[serde(rename = "gasPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<U256>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<U256>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Bytes>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Filter {
    #[serde(rename = "fromBlock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_block: Option<BlockParameter>,
    #[serde(rename = "toBlock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_block: Option<BlockParameter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<ValueOrVec<H160>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<Option<ValueOrVec<H256>>>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValueOrVec<T> {
    Value(T),
    Vec(Vec<T>),
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum HashOrLog {
    H256(H256),
    Log(Log),
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct TxPoolStatus {
    pub pending: U256,
    pub queued: U256,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct TxPoolContent {
    pending: HashMap<H160, HashMap<String, Transaction>>,
    queued: HashMap<H160, HashMap<String, Transaction>>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct TxPoolInspect {
    pending: HashMap<H160, HashMap<String, String>>,
    queued: HashMap<H160, HashMap<String, String>>,
}
