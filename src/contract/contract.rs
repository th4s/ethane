use crate::types::ContractCall;

use anyhow::anyhow;

//use crate::rpc;
//use crate::types::{
  //  BlockParameter, Bytes, Call, Filter, GasCall, TransactionRequest, ValueOrVec, H256, U256, U64, H160,
//};

pub fn query(call: ContractCall) {
    let encoded = encode_input(serde_json::to_vec(&call.abi).unwrap().as_slice(), "solution", &[], false).unwrap();
    println!("Lofasz: {}", encoded);
//     let call = Call {
//         to: address,
//         data: Some(Bytes::from_slice(encoded.as_bytes())),
//         ..Default::default()
//     };
//
//     rpc::eth_call(call, None);
//     //let abi = ethabi::Contract::load(json).unwrap();
//     //abi.function("").unwrap().encode_input(&params.into_tokens()).unwrap();
}

fn load_function(abi_json: &[u8], name_or_signature: &str) -> anyhow::Result<ethabi::Function> {
    //let file = File::open(path)?;
    let contract = ethabi::Contract::load(abi_json)?;
    let params_start = name_or_signature.find('(');

    match params_start {
        // It's a signature
        Some(params_start) => {
            let name = &name_or_signature[..params_start];

            contract
                .functions_by_name(name)?
                .iter()
                .find(|f| f.signature() == name_or_signature)
                .cloned()
                .ok_or_else(|| anyhow!("invalid function signature `{}`", name_or_signature))
        }

        // It's a name
        None => {
            let functions = contract.functions_by_name(name_or_signature)?;
            match functions.len() {
                0 => unreachable!(),
                1 => Ok(functions[0].clone()),
                _ => Err(anyhow!(
					"More than one function found for name `{}`, try providing the full signature",
					name_or_signature
				)),
            }
        }
    }
}

fn encode_input(abi_json: &[u8], name_or_signature: &str, values: &[String], lenient: bool) -> anyhow::Result<String> {
    let function = load_function(abi_json, name_or_signature).unwrap();

    let params: Vec<_> =
        function.inputs.iter().map(|param| param.kind.clone()).zip(values.iter().map(|v| v as &str)).collect();

    let tokens = parse_tokens(&params, lenient)?;
    let result = function.encode_input(&tokens)?;

    Ok(hex::encode(&result))
}

use ethabi::{
    token::{LenientTokenizer, StrictTokenizer, Token, Tokenizer},
};

fn parse_tokens(params: &[(ethabi::ParamType, &str)], lenient: bool) -> anyhow::Result<Vec<Token>> {
    params
        .iter()
        .map(|&(ref param, value)| match lenient {
            true => LenientTokenizer::tokenize(param, value),
            false => StrictTokenizer::tokenize(param, value),
        })
        .collect::<Result<_, _>>()
        .map_err(From::from)
}