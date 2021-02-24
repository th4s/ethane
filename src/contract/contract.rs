use crate::types::{ContractCall, Call, Bytes, U256};

use anyhow::anyhow;

use crate::rpc;

pub fn query(call: ContractCall) -> Rpc<Bytes> {
    let encoded = encode_input(serde_json::to_vec(&call.abi).unwrap().as_slice(), "solution", &[], false).unwrap();
    println!("Request: {:?}", encoded);

    let tr = Call{
        //from: Some(call.from),
        to: call.to,
        data: Some(encoded),
        //data: Some(Bytes::from_slice(encoded.as_bytes())),
        //gas: Some(U256::from(250000)),
        ..Default::default()
    };
    let result = rpc::eth_call(tr, None);
    //let result = rpc::eth_call(call, None);
    println!("Request wrapped: {:?}", result.params);
    result
//     //let abi = ethabi::Contract::load(json).unwrap();
//     //abi.function("").unwrap().encode_input(&params.into_tokens()).unwrap();
}

pub fn fetch_query_result(call: ContractCall, b: Bytes) {
    decode_call_output(serde_json::to_vec(&call.abi).unwrap().as_slice(), "solution", b);
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

fn encode_input(abi_json: &[u8], name_or_signature: &str, values: &[String], lenient: bool) -> anyhow::Result<Bytes> {
    let function = load_function(abi_json, name_or_signature).unwrap();

    let params: Vec<_> =
        function.inputs.iter().map(|param| param.kind.clone()).zip(values.iter().map(|v| v as &str)).collect();

    let tokens = parse_tokens(&params, lenient)?;
    let result = function.encode_input(&tokens)?;
    println!("Function: {:?}", result);
    //img.iter().flat_map(|rgb| rgb.data.iter()).cloned().collect();
    Ok(Bytes::from_slice(result.as_slice()))
}

use ethabi::{
    token::{LenientTokenizer, StrictTokenizer, Token, Tokenizer},
};
use crate::rpc::Rpc;
use ethereum_types::H256;

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

fn decode_call_output(abi_json: &[u8], name_or_signature: &str, data: Bytes) -> anyhow::Result<String> {
    let function = load_function(abi_json, name_or_signature)?;
    //let data: Vec<u8> = hex::decode(&data)?;
    let tokens = function.decode_output(data.0.as_slice())?;
    let types = function.outputs;

    assert_eq!(types.len(), tokens.len());

    let result = types
        .iter()
        .zip(tokens.iter())
        .map(|(ty, to)| format!("{} {}", ty.kind, to))
        .collect::<Vec<String>>()
        .join("\n");

    println!("Solution: {}", result);
    Ok(result)
}