//! Ethane is an alternative web3 implementation with the aim of being slim and simple.
//! It does not depend on futures, async/await or any executors.
//!
//! # Examples
//!
//! ## Request over http
//! ```
//! use ethane::Connector;
//! use ethane::rpc::eth_get_balance;
//! use ethane::types::H160;
//! # use test_helper::NodeProcess;
//! # let node = NodeProcess::new(Some(8545), Some(8547));
//!
//! // Start up connector
//! let node_endpoint = "http://127.0.0.1:8545";
//! let mut connector = Connector::http(node_endpoint, &None).unwrap();
//!
//! // Make a request
//! let address = H160::zero();
//! let balance = connector.call(eth_get_balance(address, None)).unwrap();
//! ```
//!
//! ## Starting a subscription over websocket
//! ```
//! use ethane::Connector;
//! use ethane::rpc::sub::eth_subscribe_new_pending_transactions;
//! # use test_helper::NodeProcess;
//! # use ethane::rpc::{eth_send_transaction, eth_coinbase};
//! # use ethane::types::{TransactionRequest, H160, U256};
//!
//! # let node = NodeProcess::new(Some(8544), Some(8546));
//!
//! // Start up connector with websockets
//! let node_endpoint = "ws://127.0.0.1:8546";
//! let mut connector = Connector::websocket(node_endpoint, &None).unwrap();
//!
//! // Subscribe to pending transactions
//! let mut tx_subscription = connector
//!     .subscribe(eth_subscribe_new_pending_transactions()).unwrap();
//! # let tx_request = TransactionRequest {
//! # from: connector.call(eth_coinbase()).unwrap(),
//! # to: Some(H160::zero()),
//! # value: Some(U256::zero()),
//! # ..Default::default()
//! # };
//! # let _tx_hash = connector.call(eth_send_transaction(tx_request));
//!
//! // Get next transaction item
//! let tx = tx_subscription.next_item().unwrap();
//! ```

pub use connector::{Connector, ConnectorError};
pub use transport::Credentials;

pub mod connector;
pub mod rpc;
pub mod transport;
pub mod types;
