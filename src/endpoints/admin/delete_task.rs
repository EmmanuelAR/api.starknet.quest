use crate::models::{JWTClaims, QuestTaskDocument};
use crate::utils::verify_task_auth;
use crate::{models::AppState, utils::get_error};
use axum::http::HeaderMap;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use axum_auto_routes::route;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use mongodb::bson::doc;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

pub_struct!(Deserialize; DeleteTask {
   id: i32,
});

#[route(post, "/admin/tasks/remove_task")]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    body: Json<DeleteTask>,
) -> impl IntoResponse {
    let user = check_authorization!(headers, &state.conf.auth.secret_key.as_ref()) as String;
    let collection = state.db.collection::<QuestTaskDocument>("tasks");
    let res = verify_task_auth(user, &collection, &body.id).await;
    if !res {
        return get_error("Error updating tasks".to_string());
    }

    // filter to get existing boost
    let filter = doc! {
        "id": &body.id,
    };
    return match &collection.delete_one(filter.clone(), None).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({"message": "deleted successfully"})),
        )
            .into_response(),
        Err(_) => {
            return get_error("Task does not exist".to_string());
        }
    };
}
