use crate::{AppError, AppState, CreateChat, UpdateChat};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use chat_core::User;

#[utoipa::path(
    get,
    path = "/api/chats",
    responses(
        (status = 200, description = "List of chats", body = Vec<Chat>),
    ),
    security(
        ("token" = [])
    ),tag = "chat"
)]
pub(crate) async fn list_chat_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let chat = state.fetch_chats(user.ws_id as _).await?;
    Ok((StatusCode::OK, Json(chat)))
}

#[utoipa::path(
    post,
    path = "/api/chats",
    responses(
        (status = 201, description = "Chat created", body = Chat),
    ),
    security(
        ("token" = [])
    ),tag = "chat"
)]
pub(crate) async fn create_chat_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(input): Json<CreateChat>,
) -> Result<impl IntoResponse, AppError> {
    let chat = state.create_chat(input, user.ws_id as _).await?;
    Ok((StatusCode::CREATED, Json(chat)))
}

#[utoipa::path(
    get,
    path = "/api/chats/{id}",
    params(
        ("id" = u64, Path, description = "Chat id")
    ),
    responses(
        (status = 200, description = "Chat found", body = Chat),
        (status = 404, description = "Chat not found", body = ErrorOutput),
    ),
    security(
        ("token" = [])
    ),tag = "chat"
)]
pub(crate) async fn get_chat_handler(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<impl IntoResponse, AppError> {
    let chat = state.get_chat_by_id(id as _).await?;
    match chat {
        Some(chat) => Ok(Json(chat)),
        None => Err(AppError::NotFound(format!("chat id {id}"))),
    }
}
#[utoipa::path(
    patch,
    path = "/api/chats/{id}",
    params(
        ("id" = u64, Path, description = "Chat id")
    ),
    responses(
        (status = 200, description = "Chat update success"),
        (status = 404, description = "Chat not found", body = ErrorOutput),
    ),
    security(
        ("token" = [])
    ),
    tag = "chat"
)
]
pub(crate) async fn update_chat_handler(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(input): Json<UpdateChat>,
) -> Result<impl IntoResponse, AppError> {
    let flag = state.update_chat_name(id, input).await?;
    if flag {
        Ok(StatusCode::OK)
    } else {
        Err(AppError::NotFound(format!("chat id {id}")))
    }
}
#[utoipa::path(
    delete,
    path = "/api/chat/{id}",
    params(
        ("id" = u64, Path, description = "Chat id")
    ),
    responses(
        (status = 200, description = "delete chat success"),
        (status = 404, description = "Chat not found", body = ErrorOutput),
    ),
    security(
        ("token" = [])
    ),
    tag = "chat",
)]
pub(crate) async fn delete_chat_handler(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<impl IntoResponse, AppError> {
    let flag = state.delete_chat_by_id(id).await?;
    if flag {
        Ok(StatusCode::OK)
    } else {
        Err(AppError::NotFound(format!("chat id {id}")))
    }
}
