#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, DepsMut, Env, MessageInfo, Response, CosmosMsg, WasmMsg, IbcMsg};
use cw2::set_contract_version;
use serde_json::Value;
use std::collections::HashMap;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:terra-test-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::HydrateMessage { msg, vars } => execute_hydrate_msg(msg, vars),
    }
}

/*
 * Description
 * Hydrate given message with given variables.
 * Deserialize the standard CosmosMsg with serde_json_wasm since it is strictly typed.
 * Call recursively_hydrate function to hydrate all the encoded sub-messages.
 * 
 * Param            Type                        Description
 * msg              String                      String to hydrate.
 * vars             String                      String of variable names and values to use during replacement.
 * 
 * Return value: Result<Response, ContractError>
 * On success contains the CosmosMsg with hydrations already applied.
*/
fn execute_hydrate_msg(msg: String, vars: String) -> Result<Response,ContractError>{

    // Replace ':' with ',' to create vec where every 2 elements are a key-value pair
    let vars: String = vars.replace(":",",");

    // Deserialize vars string into Vec of Strings
    let vars_vec: Vec<String> = serde_json_wasm::from_str(&vars).expect("Failed deserialize vars input string into a Vec of Strings");
    
    // Insert the variable name-value pairs into a hashmap
    let var_pairs: HashMap<String, String> = vars_vec.chunks_exact(2)
                                    .map(|chunk| (chunk[0].clone(),chunk[1].clone()))
                                    .collect::<HashMap<String,String>>();

    // Hydrate the initial message
    let msg = replace_vars(msg, &var_pairs);
    
    // Deserialize first level using serde_json_wasm into a CosmosMsg
    let mut cosmos_msg: CosmosMsg = serde_json_wasm::from_str(&msg).expect("Failed to deserialize String into CosmosMsg");

    // Start recursively hydrating the base64 encoded messages.
    // All possible Binary fields are managed and hydrated in the match statement.
    match &mut cosmos_msg {
        CosmosMsg::Wasm(wasm_msg) => {
            match wasm_msg {
                WasmMsg::Execute{contract_addr:_, msg, funds:_} => {
                    *msg = recursively_hydrate(msg, &var_pairs).expect("Failed to hydrate msg field");
                },
                WasmMsg::Instantiate{admin:_,code_id:_,msg, funds:_, label:_} => {
                    *msg = recursively_hydrate(msg, &var_pairs).expect("Failed to hydrate msg field");
                },
                WasmMsg::Instantiate2{admin:_, code_id:_, label:_, msg, funds:_, salt} => {
                    *msg = recursively_hydrate(msg, &var_pairs).expect("Failed to hydrate msg field");
                    *salt = recursively_hydrate(salt, &var_pairs).expect("Failed to hydrate salt field");
                },
                WasmMsg::Migrate{contract_addr:_, new_code_id:_, msg} => {
                    *msg = recursively_hydrate(msg, &var_pairs).expect("Failed to hydrate msg field");
                },
                _ => {}
            }
        },
        CosmosMsg::Ibc(ibc_msg) => {
            match ibc_msg {
                IbcMsg::SendPacket { channel_id:_, data, timeout:_ } => {
                    *data = recursively_hydrate(data, &var_pairs).expect("Failed to hydrate data field");
                },
                _ => {}
            }
        },
        CosmosMsg::Stargate { type_url:_, value } => {
            *value = recursively_hydrate(value, &var_pairs).expect("Failed to hydrate value field");
        }
        _ => {}
    }

    // Add hydrated CosmosMsg to Response
    Ok(Response::new()
        .add_message(cosmos_msg)
        .add_attribute("action", "hydrate")
    )
}

/*
 * Description
 * Recursively hydrates the base64 encoded message and all encoded sub messages.
 * 
 * Param            Type                        Description
 * msg              &mut Binary                 Binary msg than needs to be decoded, hydrated, and encoded.
 * var_pairs        &HashMap<String,String>     Map of variable name-value pairs.
 * 
 * Return value: Result<Binary, ContractError>
 * On success contains the encoded msg with hydrations already applied.
*/
fn recursively_hydrate(msg: &mut Binary, var_pairs: &HashMap<String, String>) -> Result<Binary, ContractError>{
    
    // Decode string from base64 encoding
    let mut str_msg: String = String::from_utf8(msg.as_slice().to_vec()).expect("failed to decode");
    
    str_msg = replace_vars(str_msg, var_pairs);

    // Deserialize using generic deserializer serde_json
    let mut json_value: Value = serde_json::from_str(str_msg.as_str()).expect("failed to deserialize");

    // DFS through the json value and find and replace properties that need to be binary decoded
    find_and_replace_binary_fields(&mut json_value, var_pairs);
    
    // Serialize json value back into string
    str_msg = serde_json::to_string(&json_value).expect("failed to serialize");

    // convert string into Binary and return
    Ok(Binary::from(str_msg.into_bytes()))
}

/*
 * Description
 * Searches the Value tree, using DFS, to find all sub messages with binary values. 
 * When a encoded message is found the recursively_hydrate function is called on it.
 * 
 * Param            Type                        Description
 * val              &mut Value                  Dynamic enum of json contents
 * var_pairs        &HashMap<String,String>     Map of variable name-value pairs.
 * 
 * Return value: void
*/
fn find_and_replace_binary_fields(val: &mut Value, var_pairs: &HashMap<String, String>){
    // Matches the value to an Object (cotain key-value pairs), a Vector, or a String (all other types disregarded).
    match val {
        Value::Object(map) => {
            for (_, v) in map.iter_mut(){
                find_and_replace_binary_fields(v, var_pairs);
            }
        },
        Value::Array(vec) => {
            for v in vec {
                find_and_replace_binary_fields(v, var_pairs);
            }
        },
        Value::String(s) if Binary::from_base64(s).is_ok() => {
            // Exract binary string from base64 encoding.
            let mut binary_str: Binary = Binary::from_base64(s).expect("Failed to decode from base64");
            // Hydrate binary string by calling the recursively_hydrate function.
            let binary_new: Binary = recursively_hydrate(&mut binary_str, var_pairs).expect("Failed to hydrate inner message");
            // Convert new binary string (already hydrated) into a String
            let string_new: String = Binary::to_base64(&binary_new);
            // Override the value with the new string
            *val = Value::String(string_new);
        },
        _ => {}
    }
}

/*
 * Description
 * Replace var names with their respective values. 
 * 
 * Param            Type                        Description
 * msg              String                      String to be hydrated.
 * var_pairs        &HashMap<String,String>     Map of variable name-value pairs.
 * 
 * Return value: String
 * The string with replacements performed.
*/
fn replace_vars (msg: String, var_pairs: &HashMap<String, String>) -> String{
    let mut mut_msg: String = msg;

    // Replace variables with their values
    for (var_name, var_val) in var_pairs{
        mut_msg = mut_msg.replace(var_name,var_val);
    }
    return mut_msg;
}