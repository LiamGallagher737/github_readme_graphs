use crate::endpoints::weekly_commit_count::weekly_commit_count;
use axum::{routing::get, Router};
use reqwest::Client;
use shuttle_secrets::SecretStore;
use tera::Tera;

mod endpoints;
mod graph;

#[derive(Clone)]
pub struct App {
    secrets: Secrets,
    http_client: Client,
    tera: Tera,
}

#[derive(Clone)]
pub struct Secrets {
    github_token: String,
}

#[shuttle_runtime::main]
async fn axum(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> shuttle_axum::ShuttleAxum {
    let Some(github_token) = secret_store.get("GITHUB_TOKEN") else {
        panic!("No GitHub token provided");
    };

    let secrets = Secrets { github_token };

    let http_client = Client::new();

    let mut tera = Tera::default();
    tera.add_raw_template("graph", include_str!("graph.tera.svg"))
        .expect("Failed to add graph template");

    let app = App {
        secrets,
        http_client,
        tera,
    };

    let router = Router::new()
        .route(
            "/repo/:user/:repo/weekly-commit-count",
            get(weekly_commit_count),
        )
        .with_state(app);

    Ok(router.into())
}
