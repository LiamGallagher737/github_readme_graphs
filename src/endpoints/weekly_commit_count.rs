use super::fetch_github;
use crate::graph::{Graph, Vec2};
use crate::App;
use axum::extract::{Path, Query, State};
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct WeeklyCommitCountData {
    all: Vec<u32>,
    owner: Vec<u32>,
}

#[derive(Deserialize)]
pub struct Parameters {
    width: Option<usize>,
    height: Option<usize>,
    title: Option<String>,
    color: Option<String>,
}

pub async fn weekly_commit_count(
    Path((user, repo)): Path<(String, String)>,
    parameters: Query<Parameters>,
    State(app): State<App>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let url = format!("https://api.github.com/repos/{user}/{repo}/stats/participation");
    let data = fetch_github::<WeeklyCommitCountData>(url, &app).await?;

    let title = parameters
        .title
        .clone()
        .unwrap_or("Weekly Commit Count".to_string());

    let color = parameters.color.clone().unwrap_or("#99c1f1".to_string());

    let graph = Graph {
        title,
        color,
        points: data
            .all
            .iter()
            .enumerate()
            .map(|(n, v)| Vec2::new(n as f64 + 1.0, *v as f64))
            .collect(),
    };

    let width = parameters.width.unwrap_or(800);
    let height = parameters.height.unwrap_or(400);

    let svg = graph
        .svg(&app.tera, width, height)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create SVG"))?;

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "image/svg+xml")],
        svg,
    ))
}
