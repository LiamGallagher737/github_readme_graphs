use crate::App;
use axum::http::StatusCode;
use serde::de::DeserializeOwned;

pub mod weekly_commit_count;

async fn fetch_github<T: DeserializeOwned>(
    url: String,
    app: &App,
) -> Result<T, (StatusCode, &'static str)> {
    const ISE: StatusCode = StatusCode::INTERNAL_SERVER_ERROR;
    let response = app
        .http_client
        .get(&url)
        .header("Authorization", app.secrets.github_token.clone())
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "GitHub README Graphs")
        .send()
        .await
        .map_err(|_| (ISE, "Failed to fetch GitHub data"))?;

    match response.status() {
        StatusCode::ACCEPTED => return Err((StatusCode::ACCEPTED, "Data is being generated")),
        StatusCode::NOT_FOUND => return Err((StatusCode::NOT_FOUND, "Repo not found")),
        _ => {}
    }

    let data = response
        .json::<T>()
        .await
        .map_err(|_| (ISE, "Failed to parse GitHub data"))?;
    Ok(data)
}
