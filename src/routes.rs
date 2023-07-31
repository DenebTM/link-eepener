use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
};
use std::{collections::HashMap, fs};
use tracing::{event, Level};

use crate::{
    common::{AppState, LinkScreee},
    disk_store, screee,
};

pub async fn index() -> impl IntoResponse {
    match fs::read_to_string("./index.html") {
        Ok(content) => Html(content).into_response(),
        Err(err) => (StatusCode::NOT_FOUND, err.to_string()).into_response(),
    }
}

pub async fn create_screee(
    Query(params): Query<HashMap<String, String>>,
    State(links): AppState,
) -> Result<LinkScreee, (StatusCode, String)> {
    let maybe_url = params.get("url");

    if let Some(url) = maybe_url {
        let mut data = links.lock().await;

        let screee = match data.get_by_left(url) {
            Some(existing_screee) => {
                event!(Level::DEBUG, "Found URL in store");
                existing_screee.to_string()
            }
            None => {
                event!(Level::DEBUG, "URL not found in store");
                let mut new_screee: String = screee::generate();
                while let Some(_) = data.get_by_right(&new_screee) {
                    event!(Level::DEBUG, "Collision, retrying");
                    new_screee = screee::generate();
                }

                data.insert(url.to_string(), new_screee.clone());
                disk_store::save(&data);
                new_screee
            }
        };

        event!(Level::TRACE, "{} <=> {}", &screee, url);
        Ok([screee.as_str(), "\n"].concat())
    } else {
        event!(Level::DEBUG, "'url' parameter missing in query");
        Err((
            StatusCode::BAD_REQUEST,
            "Missing required query parameter 'url'\n".to_string(),
        ))
    }
}

pub async fn use_screee(Path(screee): Path<LinkScreee>, State(links): AppState) -> Redirect {
    let data = links.lock().await;

    let uri = match data.get_by_right(&screee) {
        Some(url) => {
            event!(Level::DEBUG, "Found Screee in store");
            event!(Level::TRACE, "{} <=> {}", &screee, url);
            url.as_str()
        }
        None => {
            event!(Level::DEBUG, "Screee not found in store");
            "/"
        }
    };

    Redirect::temporary(uri)
}
