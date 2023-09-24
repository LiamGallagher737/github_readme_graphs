use crate::endpoints::weekly_commit_count::weekly_commit_count;
use axum::{routing::get, Router};
use reqwest::{Client, StatusCode};
use serde::de::DeserializeOwned;
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
    // tera.register_filter("pretty", graph::pretty);
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

async fn fetch_github<T: DeserializeOwned>(
    url: String,
    app: &App,
) -> Result<T, (StatusCode, &'static str)> {
    const ISE: StatusCode = StatusCode::INTERNAL_SERVER_ERROR;
    let data = app
        .http_client
        .get(&url)
        .header("Authorization", app.secrets.github_token.clone())
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "GitHub README Graphs")
        .send()
        .await
        .map_err(|_| (ISE, "Failed to get GitHub data"))?
        .json::<T>()
        .await
        .map_err(|_| (ISE, "Failed to parse GitHub data"))?;
    Ok(data)
}
