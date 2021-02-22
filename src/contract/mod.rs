
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
    abi.function("").unwrap().encode_input(&params.into_tokens()).unwrap();
}

fn load_function(abi_json: &[u8], name_or_signature: &str) -> anyhow::Result<ethane::Function> {
    //let file = File::open(path)?;
    let contract = ethane::Contract::load(abi_json)?;
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
    let function = load_function(abi_json, name_or_signature)?;

    let params: Vec<_> =
        function.inputs.iter().map(|param| param.kind.clone()).zip(values.iter().map(|v| v as &str)).collect();

    let tokens = parse_tokens(&params, lenient)?;
    let result = function.encode_input(&tokens)?;

    Ok(hex::encode(&result))
}

