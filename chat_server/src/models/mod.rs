mod chat;
mod file;
mod messages;
mod user;
mod workspace;

pub use chat::{CreateChat, UpdateChat};
pub use messages::{CreateMessage, ListMessages};
use serde::{Deserialize, Serialize};
pub use user::{CreateUser, SigninUser};
use utoipa::ToSchema;

#[derive(Debug, ToSchema, Clone, Serialize, Deserialize)]
pub struct ChatFile {
    pub ws_id: u64,
    pub ext: String, // extract ext from filename or mime type
    pub hash: String,
}
