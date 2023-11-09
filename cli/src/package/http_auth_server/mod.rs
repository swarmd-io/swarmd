use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;

use anyhow::bail;
use axum::extract::{Query, State};
use axum::routing::get;
use axum::Router;
mod response;
use axum::response::{IntoResponse, Redirect};
use response::TokenQueryParams;
use tokio::sync::mpsc::{self, Sender};
use tracing::info;

#[derive(Debug)]
pub struct HttpAuthServer {
    port: u16,
}

const AUTH_URL: &str = "http://localhost:3000/auth/success";

#[derive(Clone)]
struct HttpAuthState {
    token_tx: Arc<Sender<String>>,
}

impl HttpAuthServer {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    /// Create a new server to get the token locally
    pub async fn get_token(&self, timeout: Duration) -> anyhow::Result<String> {
        let (shutdown_tx, mut shutdown_rx) = mpsc::channel::<()>(8);
        let (token_tx, mut token_rx) = mpsc::channel::<String>(2);

        let router = Router::new().route("/", get(token));

        let socket_address = SocketAddr::from((Ipv4Addr::LOCALHOST, self.port));

        let state = HttpAuthState {
            token_tx: Arc::new(token_tx),
        };

        let token_handle = tokio::spawn(async move {
            if let Some(token) = token_rx.recv().await {
                let _ = shutdown_tx.try_send(());
                Ok(token)
            } else {
                bail!("Couldn't get the token.");
            }
        });

        let server =
            axum::Server::bind(&socket_address).serve(router.with_state(state).into_make_service());
        let fut = tokio::time::timeout(
            timeout,
            server.with_graceful_shutdown(async move {
                shutdown_rx.recv().await;
            }),
        )
        .await;

        match fut {
            Ok(Ok(_)) => {
                if let Ok(Ok(Ok(token))) =
                    tokio::time::timeout(Duration::from_secs(1), token_handle).await
                {
                    Ok(token)
                } else {
                    bail!("Something wrong happened");
                }
            }
            Ok(Err(err)) => {
                bail!(err);
            }
            Err(elapsed) => {
                bail!("Timeout after {elapsed:?}");
            }
        }
    }
}

async fn token<'a>(
    State(state): State<HttpAuthState>,
    query: Query<TokenQueryParams>,
) -> impl IntoResponse {
    info!("Token received.");
    let token = &query.token;

    let _ = state.token_tx.try_send(token.clone()).inspect_err(|err| {
        tracing::error!("{err:?}");
    });

    Redirect::temporary(&format!("{AUTH_URL}"))
}
