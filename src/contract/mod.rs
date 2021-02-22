
// /// Ethereum Contract Interface
// #[derive(Debug, Clone)]
// pub struct Contract {
//     // address: Address,
//     // eth: Eth<T>,
//     abi: ethabi::Contract,
// }
//
// impl Contract {
//     fn new() -> ethabi::Result<Self> {
//         let abi = ethabi::Contract::load(json)?;
//         Ok(Contract {
//             abi,
//         })
//     }
//
//
// }

pub fn query(json: &[u8]) {
    let abi = ethabi::Contract::load(json).unwrap();
    abi.function("").unwrap().encode_input(&params.into_tokens()).map(|call| (call, function));
}

