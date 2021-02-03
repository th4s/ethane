//! Ethane is an alternative web3 implementation with the aim of being slim and simple. It uses a
//! blocking api and does not depend on futures, async/await or any executors.
//!
//! # Example
//! ```
//! use ethane::Connector;
//! use ethane::rpc::eth_get_balance;
//! #
//!
//!
//! #
//! // Substitute with your node endpoint
//! let connector = Connector::http("127.0.0.1:8545", &None).unwrap();
//! let balance = connector.
//!
//! ```

pub use connector::{Connector, ConnectorError};
pub use transport::Credentials;

pub mod connector;
pub mod rpc;
pub mod transport;
pub mod types;
