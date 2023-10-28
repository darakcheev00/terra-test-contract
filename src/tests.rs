use crate::contract::{execute, instantiate};
use crate::msg::{ExecuteMsg, InstantiateMsg};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{attr, CosmosMsg};

#[test]
fn test_instantiate() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("addr_admin", &[]);
    let msg = InstantiateMsg {};

    let resp = instantiate(deps.as_mut(), env, info, msg).unwrap();
    assert_eq!(resp.attributes, vec![attr("action", "instantiate")]);
}

#[test]
fn test_hydrate_msg_wasm_execute() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("addr_admin", &[]);
    let msg = InstantiateMsg {};

    let _resp: cosmwasm_std::Response =
        instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Msg with variable placeholders (unhydrated)
    let cosmos_msg1: String = "{\"wasm\":{\"execute\":{\"contract_addr\":\"$warp.var.variable1\",\"msg\":\"eyJzZW5kIjp7ImNvbnRyYWN0IjoidGVycmE1NDMyMSIsImFtb3VudCI6IjEyMzQ1IiwibXNnIjoiZXlKbGVHVmpkWFJsWDNOM1lYQmZiM0JsY21GMGFXOXVjeUk2ZXlKdmNHVnlZWFJwYjI1eklqcGJleUpoYzNSeWIxOXpkMkZ3SWpwN0ltOW1abVZ5WDJGemMyVjBYMmx1Wm04aU9uc2lkRzlyWlc0aU9uc2lZMjl1ZEhKaFkzUmZZV1JrY2lJNklpUjNZWEp3TG5aaGNpNTJZWEpwWVdKc1pURWlmWDBzSW1GemExOWhjM05sZEY5cGJtWnZJanA3SW01aGRHbDJaVjkwYjJ0bGJpSTZleUprWlc1dmJTSTZJaVIzWVhKd0xuWmhjaTUyWVhKcFlXSnNaVElpZlgxOWZWMHNJbTFwYm1sdGRXMWZjbVZqWldsMlpTSTZJaVIzWVhKd0xuWmhjaTUyWVhKcFlXSnNaVE1pTENKMGJ5STZJaVIzWVhKd0xuWmhjaTUyWVhKcFlXSnNaVFFpTENKdFlYaGZjM0J5WldGa0lqb2lKSGRoY25BdWRtRnlMblpoY21saFlteGxOU0o5ZlE9PSJ9fQ==\",\"funds\":[]}}}"
            .to_string();

    // String containing variable name-value pairs
    let test_vars: String = r#"[
                            "$warp.var.variable1": "terra12345",
                            "$warp.var.variable2": "uterra",
                            "$warp.var.variable3": "54321",
                            "$warp.var.variable4": "terra11111",
                            "$warp.var.variable5": "0.05"
                        ]"#
    .to_string();

    // Create message to hydrate the string
    let msg = ExecuteMsg::HydrateMsg {
        msg: cosmos_msg1.clone(),
        vars: test_vars,
    };

    // Execute message and retrieve response containing hydrated CosmosMsg
    let resp: cosmwasm_std::Response =
        execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Construct ground truth manually
    let level3: String = "{\"execute_swap_operations\":{\"max_spread\":\"0.05\",\"minimum_receive\":\"54321\",\"operations\":[{\"astro_swap\":{\"ask_asset_info\":{\"native_token\":{\"denom\":\"uterra\"}},\"offer_asset_info\":{\"token\":{\"contract_addr\":\"terra12345\"}}}}],\"to\":\"terra11111\"}}".to_string();
    let level3_enc: String = base64::encode(level3);

    let mut level2: String =
        "{\"send\":{\"amount\":\"12345\",\"contract\":\"terra54321\",\"msg\":\"PLACEHOLDER\"}}"
            .to_string();
    level2 = level2.replace("PLACEHOLDER", &level3_enc);
    let level2_enc: String = base64::encode(level2);

    let level1: String = "{\"wasm\":{\"execute\":{\"contract_addr\":\"terra12345\",\"msg\":\"PLACEHOLDER\",\"funds\":[]}}}".to_string();
    let cosmos_msg_answer_str: String = level1.replace("PLACEHOLDER", &level2_enc);

    // Create CosmosMsg for the Ground truth with which the response will be compared with by deserializing the string.
    let ground_truth_cosmos_msg: CosmosMsg = serde_json_wasm::from_str(&cosmos_msg_answer_str)
        .expect("failed to deserialize ground truth");

    // Assert the CosmosMsg in response is correctly hydrated.
    assert_eq!(resp.messages[0].msg, ground_truth_cosmos_msg);
    assert_eq!(resp.attributes, vec![attr("action", "hydrate")]);
}

#[test]
fn test_hydrate_msg_ibc_send_packet() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("addr_admin", &[]);
    let msg = InstantiateMsg {};

    let _resp: cosmwasm_std::Response =
        instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Msg with variable placeholders (unhydrated)
    let cosmos_msg1: String = "{\"ibc\":{\"send_packet\":{\"channel_id\":\"$warp.var.variable1\",\"data\":\"eyJzZW5kIjp7ImNvbnRyYWN0IjoidGVycmE1NDMyMSIsImFtb3VudCI6IjEyMzQ1IiwibXNnIjoiZXlKbGVHVmpkWFJsWDNOM1lYQmZiM0JsY21GMGFXOXVjeUk2ZXlKdmNHVnlZWFJwYjI1eklqcGJleUpoYzNSeWIxOXpkMkZ3SWpwN0ltOW1abVZ5WDJGemMyVjBYMmx1Wm04aU9uc2lkRzlyWlc0aU9uc2lZMjl1ZEhKaFkzUmZZV1JrY2lJNklpUjNZWEp3TG5aaGNpNTJZWEpwWVdKc1pURWlmWDBzSW1GemExOWhjM05sZEY5cGJtWnZJanA3SW01aGRHbDJaVjkwYjJ0bGJpSTZleUprWlc1dmJTSTZJaVIzWVhKd0xuWmhjaTUyWVhKcFlXSnNaVElpZlgxOWZWMHNJbTFwYm1sdGRXMWZjbVZqWldsMlpTSTZJaVIzWVhKd0xuWmhjaTUyWVhKcFlXSnNaVE1pTENKMGJ5STZJaVIzWVhKd0xuWmhjaTUyWVhKcFlXSnNaVFFpTENKdFlYaGZjM0J5WldGa0lqb2lKSGRoY25BdWRtRnlMblpoY21saFlteGxOU0o5ZlE9PSJ9fQ==\",\"timeout\":{\"block\":null,\"timestamp\":\"$warp.var.variable3\"}}}}"
            .to_string();

    // String containing variable name-value pairs
    let test_vars: String = r#"[
                            "$warp.var.variable1": "terra12345",
                            "$warp.var.variable2": "uterra",
                            "$warp.var.variable3": "54321",
                            "$warp.var.variable4": "terra11111",
                            "$warp.var.variable5": "0.05"
                        ]"#
    .to_string();

    // Create message to hydrate the string
    let msg = ExecuteMsg::HydrateMsg {
        msg: cosmos_msg1.clone(),
        vars: test_vars,
    };

    // Execute message and retrieve response containing hydrated CosmosMsg
    let resp: cosmwasm_std::Response =
        execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Construct ground truth manually
    let level3: String = "{\"execute_swap_operations\":{\"max_spread\":\"0.05\",\"minimum_receive\":\"54321\",\"operations\":[{\"astro_swap\":{\"ask_asset_info\":{\"native_token\":{\"denom\":\"uterra\"}},\"offer_asset_info\":{\"token\":{\"contract_addr\":\"terra12345\"}}}}],\"to\":\"terra11111\"}}".to_string();
    let level3_enc: String = base64::encode(level3);

    let mut level2: String =
        "{\"send\":{\"amount\":\"12345\",\"contract\":\"terra54321\",\"msg\":\"PLACEHOLDER\"}}"
            .to_string();
    level2 = level2.replace("PLACEHOLDER", &level3_enc);
    let level2_enc: String = base64::encode(level2);

    let level1: String = "{\"ibc\":{\"send_packet\":{\"channel_id\":\"terra12345\",\"data\":\"PLACEHOLDER\",\"timeout\":{\"block\":null,\"timestamp\":\"54321\"}}}}".to_string();
    let cosmos_msg_answer_str: String = level1.replace("PLACEHOLDER", &level2_enc);

    // Create CosmosMsg for the Ground truth with which the response will be compared with by deserializing the string.
    let ground_truth_cosmos_msg: CosmosMsg = serde_json_wasm::from_str(&cosmos_msg_answer_str)
        .expect("failed to deserialize ground truth");

    // Assert the CosmosMsg in response is correctly hydrated.
    assert_eq!(resp.messages[0].msg, ground_truth_cosmos_msg);
    assert_eq!(resp.attributes, vec![attr("action", "hydrate")]);

}
