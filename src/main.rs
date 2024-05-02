#![allow(clippy::diverging_sub_expression)]

use std::{env, sync::Arc, time::Duration};

use anyhow::Result;
use axum::{
    body::Bytes, extract::State, http::StatusCode, response::IntoResponse, routing::post, Json,
    Router,
};
use axum_extra::TypedHeader;
use commands::{handle_auto_complete, handle_command};
use ed25519_dalek::{Verifier, VerifyingKey, PUBLIC_KEY_LENGTH};
use headers::{XSignatureEd25519, XSignatureTimestamp};
use hex::FromHex;
use tokio::{net::TcpListener, signal};
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};
use tracing_subscriber::{fmt, prelude::*, EnvFilter, Registry};
use twilight_model::{
    application::interaction::{Interaction, InteractionData, InteractionType},
    http::interaction::{InteractionResponse, InteractionResponseType},
};

pub mod commands;
pub mod headers;
pub mod wikipedia;

struct AppState {
    public_key: VerifyingKey,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    Registry::default()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "discord_wikipedia=debug,tower_http=debug,axum=trace".into()),
        )
        .with(fmt::layer())
        .init();
    tracing::info!("starting up");

    let public_key = env::var("DISCORD_PUBLIC_KEY").expect("DISCORD_PUBLIC_KEY must be set");
    let state = Arc::new(AppState {
        public_key: VerifyingKey::from_bytes(
            &<[u8; PUBLIC_KEY_LENGTH] as FromHex>::from_hex(&public_key).unwrap(),
        )
        .unwrap(),
    });

    let app = Router::new()
        .route("/api/interaction", post(handle_interaction))
        .with_state(state)
        .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::new(Duration::from_secs(10)),
        ));

    let port = env::var("PORT").unwrap_or_else(|_| "10278".to_string());
    let listener = TcpListener::bind(format!("127.0.0.1:{port}"))
        .await
        .unwrap();

    let local_addr = listener.local_addr().expect("failed to get local address");
    tracing::info!("listening on http://{local_addr}");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    Ok(())
}

#[axum::debug_handler]
async fn handle_interaction(
    TypedHeader(timestamp): TypedHeader<XSignatureTimestamp>,
    TypedHeader(signature): TypedHeader<XSignatureEd25519>,
    State(state): State<Arc<AppState>>,
    body: Bytes,
) -> impl IntoResponse {
    if state
        .public_key
        .verify(
            [timestamp.0.as_bytes(), &body].concat().as_ref(),
            &signature.0.parse().expect("failed to parse signature"),
        )
        .is_err()
    {
        return (StatusCode::UNAUTHORIZED, "invalid signature").into_response();
    }

    let interaction = match serde_json::from_slice::<Interaction>(&body) {
        Ok(interaction) => interaction,
        Err(e) => {
            tracing::error!("failed to parse interaction: {e:#?}");
            return (StatusCode::BAD_REQUEST, "invalid interaction").into_response();
        }
    };

    match interaction.kind {
        InteractionType::Ping => (
            StatusCode::OK,
            Json(&InteractionResponse {
                kind: InteractionResponseType::Pong,
                data: None,
            }),
        )
            .into_response(),
        InteractionType::ApplicationCommand => {
            let data = match interaction.data {
                Some(InteractionData::ApplicationCommand(data)) => Some(data),
                _ => None,
            }
            .expect("`InteractionType::ApplicationCommand` has data");

            match handle_command(data).await {
                Ok(res) => (StatusCode::OK, Json(&res)).into_response(),
                Err(e) => {
                    tracing::error!("failed to handle command: {e:#?}");
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "failed to handle command",
                    )
                        .into_response()
                }
            }
        }
        InteractionType::ApplicationCommandAutocomplete => {
            let data = match interaction.data {
                Some(InteractionData::ApplicationCommand(data)) => Some(data),
                _ => None,
            }
            .expect("`InteractionType::ApplicationCommandAutocomplete` has data");

            match handle_auto_complete(data).await {
                Ok(res) => (StatusCode::OK, Json(&res)).into_response(),
                Err(e) => {
                    tracing::error!("failed to handle autocomplete: {e:#?}");
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "failed to handle autocomplete",
                    )
                        .into_response()
                }
            }
        }
        _ => (StatusCode::BAD_REQUEST, "invalid interaction type").into_response(),
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
