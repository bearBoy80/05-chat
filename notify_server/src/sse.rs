use crate::{AppEvent, AppState, UserMap};
use axum::{
    extract::State,
    response::{sse::Event, Sse},
    Extension,
};
use chat_core::User;
use futures::Stream;
use std::{convert::Infallible, time::Duration};
use tokio::sync::broadcast;
use tokio_stream::{wrappers::BroadcastStream, StreamExt};
use tracing::info;

const CHANNEL_CAPACITY: usize = 256;

pub(crate) async fn sse_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let user_id = user.id as u64;
    let users = &state.users;
    let guard = Guard(user_id, users.clone());

    let rx = if let Some(tx) = users.get(&user_id) {
        tx.subscribe()
    } else {
        let (tx, rx) = broadcast::channel(CHANNEL_CAPACITY);
        state.users.insert(user_id, tx);
        rx
    };
    info!("User {} subscribed", user_id);

    let stream = BroadcastStream::new(rx)
        .filter_map(|v| v.ok())
        .map(move |v| {
            guard.noop();
            let name = match v.as_ref() {
                AppEvent::NewChat(_) => "NewChat",
                AppEvent::AddToChat(_) => "AddToChat",
                AppEvent::RemoveFromChat(_) => "RemoveFromChat",
                AppEvent::NewMessage(_) => "NewMessage",
                AppEvent::UpdateChatName(_) => "UpdateChatName",
            };
            println!("{:?}", name);
            let v = serde_json::to_string(&v).expect("Failed to serialize event");
            Ok(Event::default().data(v).event(name))
        });
    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}
struct Guard(u64, UserMap);
impl Guard {
    fn noop(&self) {}
}
impl Drop for Guard {
    fn drop(&mut self) {
        info!("sse client:{:?} was dropped!", self.0);
        self.1.remove(&self.0);
    }
}
