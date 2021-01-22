use super::Rpc;
use crate::types::PrivateKey;
use ethereum_types::H160;

pub fn personal_import_raw_key(private_key: PrivateKey, password: String) -> Rpc<H160> {
    let mut rpc = Rpc::new("personal_importRawKey");
    rpc.add_param(private_key);
    rpc.add_param(password);
    rpc
}

pub fn personal_list_accounts() -> Rpc<Vec<H160>> {
    Rpc::new("personal_listAccounts")
}

pub fn personal_unlock_account(
    address: H160,
    password: String,
    duration: Option<u32>,
) -> Rpc<bool> {
    let mut rpc = Rpc::new("personal_unlockAccount");
    rpc.add_param(address);
    rpc.add_param(password);
    if let Some(duration) = duration {
        rpc.add_param(duration);
    }
    rpc
}
