use std::sync::Arc;

use crate::{
    models::{AppState, VerifyQuery},
    utils::{get_error, CompletedTasksTrait},
};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum_auto_routes::route;
use serde_json::json;
use starknet::{
    core::types::{BlockId, BlockTag, FieldElement, FunctionCall},
    macros::selector,
    providers::Provider,
};

#[route(get, "/quests/jediswap/verify_added_liquidity")]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<VerifyQuery>,
) -> impl IntoResponse {
    let task_id = 9;
    let addr = &query.addr;
    let mut calldata = vec![addr.clone(), state.conf.quests.jediswap.pairs.len().into()];
    calldata.append(&mut state.conf.quests.jediswap.pairs.clone());

    // get starkname from address
    let call_result = state
        .provider
        .call(
            FunctionCall {
                contract_address: state.conf.quests.jediswap.utils_contract,
                entry_point_selector: selector!("sum_balances"),
                calldata,
            },
            BlockId::Tag(BlockTag::Latest),
        )
        .await;

    match call_result {
        Ok(result) => {
            if result[0] == FieldElement::ZERO {
                get_error("You didn't deposit liquidity.".to_string())
            } else {
                match state.upsert_completed_task(query.addr, task_id).await {
                    Ok(_) => (StatusCode::OK, Json(json!({"res": true}))).into_response(),
                    Err(e) => get_error(format!("{}", e)),
                }
            }
        }
        Err(e) => get_error(format!("{}", e)),
    }
}
