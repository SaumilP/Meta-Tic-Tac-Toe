use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    routing::get,
    Router,
};
use futures_util::{SinkExt, StreamExt};
use uuid::Uuid;

use crate::game::{win_check::check_winner, Game};
use shared::protocol::*;
use crate::game::state::AppState;

use std::sync::Arc;

pub fn router() -> Router {
    let state = Arc::new(AppState::default());

    Router::new()
        .route("/", get(move |ws| ws_handler(ws, state.clone())))
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    state: Arc<AppState>,
) -> impl axum::response::IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
    let player_id = Uuid::new_v4();

    while let Some(Ok(msg)) = socket.next().await {
        if let axum::extract::ws::Message::Text(txt) = msg {
            let client_msg: ClientMessage = serde_json::from_str(&txt).unwrap();

            match client_msg {
                ClientMessage::MakeMove { x, y } => {
                    // simplified: single game demo
                    if let Some(mut game) = state.games.iter_mut().next() {
                        let cell = game.current_cell();
                        if game.board[y][x] == Cell::Empty {
                            game.board[y][x] = cell;

                            if let Some(winner) = check_winner(&game.board) {
                                let _ = socket
                                    .send(axum::extract::ws::Message::Text(
                                        serde_json::to_string(
                                            &ServerMessage::GameOver {
                                                winner: Some(winner),
                                            },
                                        )
                                        .unwrap(),
                                    ))
                                    .await;
                            }

                            game.turn = 1 - game.turn;

                            let _ = socket
                                .send(axum::extract::ws::Message::Text(
                                    serde_json::to_string(
                                        &ServerMessage::GameState(game.to_dto()),
                                    )
                                    .unwrap(),
                                ))
                                .await;
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
