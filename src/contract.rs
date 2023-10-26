#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:terra-test-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    unimplemented!()

    // we need to instantiate
    // can be just a default instatiation with a wallet or do we even need one
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

fn execute_hydrate_msg(msg: String, vars: String) -> Result<Response,ContractError>{
    // Deserialize vars from string to object

    // Make hashmap of vars
    let mut varMap: Map<String, String> = Map::new();
    // Fill map with all vars

    let encodedString = recursivelyHydrate(msg, &varMap);

    // create CosmosMsg and add it to response

    Ok(Response::new().add_attribute("action", "hydrated"))
}


fn recursivelyHydrate(msg, &varMap){
    // deserialize from string to object

    // check if fields other than msg contain vars
    // if so then hydrate

    // if msg field needs decoding
        // decodedString = decode message from base64
        // stringWithReplacedVars = recursivelyHydrate(decodedString, &varMap)
        // encodedString = encode the returned string
        // msg field = encodedString

    // serialize from object to string and return

}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
