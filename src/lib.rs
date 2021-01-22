//! Ethane is an alternative web3 implementation with the aim of being simple, slim and interoperable.
pub mod geth;
pub mod rpc;
pub mod transport;
pub mod types;

/// Used for HTTP basic authentication during the handshake request
pub struct Credentials {
    pub username: String,
    pub password: String,
}
