use axum::{
    extract::{Multipart, Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use tokio::fs;
use tracing::{info, warn};

use crate::{AppError, AppState, ChatFile, CreateMessage, ListMessages};
use chat_core::User;
#[utoipa::path(
    post,
    path = "/api/chats/{id}/messages",
    params(
        ("id" = u64, Path, description = "Chat id"),
    ),
    responses(
        (status = 200, description = "create message success",body =Message),
        (status = 400, description = "Invalid input", body = ErrorOutput),
    ),
    security(
        ("token" = [])
    ),tag = "chat",

)]
pub(crate) async fn send_message_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(input): Json<CreateMessage>,
) -> Result<impl IntoResponse, AppError> {
    let msg = state.create_message(input, id, user.id as _).await?;

    Ok((StatusCode::CREATED, Json(msg)))
}

#[utoipa::path(
    get,
    path = "/api/chats/{id}/messages",
    params(
        ("id" = u64, Path, description = "Chat id"),
        ListMessages

    ),
    responses(
        (status = 200, description = "List of messages", body = Vec<Message>),
        (status = 400, description = "Invalid input", body = ErrorOutput),
    ),
    security(
        ("token" = [])
    ),tag = "chat",
)]
pub(crate) async fn list_message_handler(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Query(input): Query<ListMessages>,
) -> Result<impl IntoResponse, AppError> {
    let messages = state.list_messages(input, id).await?;
    Ok(Json(messages))
}
#[utoipa::path(
    get,
    path = "/api/files/{ws_id}/{path}",
    params(
        ("ws_id" = u64, Path, description = "ws id"),
        ("path" = String, Path, description = "File path")
    ),
    responses(
        (status = 200, description = "show file",content_type = "application/octet-stream", body=Vec<u8>),
        (status = 400, description = "file not found", body = ErrorOutput),
    ),
    security(
        ("token" = [])
    ),tag = "chat",
)]
pub(crate) async fn file_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Path((ws_id, path)): Path<(i64, String)>,
) -> Result<impl IntoResponse, AppError> {
    if user.ws_id != ws_id {
        return Err(AppError::NotFound(
            "File doesn't exist or you don't have permission".to_string(),
        ));
    }
    let base_dir = state.config.server.base_dir.join(ws_id.to_string());
    let path = base_dir.join(path);
    if !path.exists() {
        return Err(AppError::NotFound("File doesn't exist".to_string()));
    }

    let mime = mime_guess::from_path(&path).first_or_octet_stream();
    // TODO: streaming
    let body = fs::read(path).await?;
    let mut headers = HeaderMap::new();
    headers.insert("content-type", mime.to_string().parse().unwrap());
    Ok((headers, body))
}

#[utoipa::path(
    post,
    path = "/api/upload",
    request_body(content_type = "multipart/form-data", content = Vec<u8>),
    responses(
        (status = 200, description = "show file",content_type = "application/json",body =Vec<String>),
        (status = 400, description = "file not found", body = ErrorOutput),
    ),
    security(
        ("token" = [])
    ),tag = "chat",
)]
pub(crate) async fn upload_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    let ws_id = user.ws_id as u64;
    let base_dir = &state.config.server.base_dir;
    let mut files = vec![];
    while let Some(field) = multipart.next_field().await.unwrap() {
        let filename = field.file_name().map(|name| name.to_string());
        let (Some(filename), Ok(data)) = (filename, field.bytes().await) else {
            warn!("Failed to read multipart field");
            continue;
        };

        let file = ChatFile::new(ws_id, &filename, &data);
        let path = file.path(base_dir);
        if path.exists() {
            info!("File {} already exists: {:?}", filename, path);
        } else {
            fs::create_dir_all(path.parent().expect("file path parent should exists")).await?;
            fs::write(path, data).await?;
        }
        files.push(file.url());
    }

    Ok(Json(files))
}
