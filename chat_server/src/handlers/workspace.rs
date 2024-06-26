use crate::{AppError, AppState};
use axum::{extract::State, response::IntoResponse, Extension, Json};
use chat_core::User;
#[utoipa::path(
    get,
    path = "/api/users/list",
    responses(
        (status = 200, description = "list chat users",body =ChatUser),
        (status = 400, description = "chat users not found", body = ErrorOutput),
    ),
    security(
        ("token" = [])
    ),tag = "chat",

)]
pub(crate) async fn list_chat_users_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let users = state.fetch_chat_users(user.ws_id as _).await?;
    Ok(Json(users))
}
