use std::sync::Arc;

use axum::extract::State;
use bimap::BiHashMap;
use tokio::sync::Mutex;

pub type LinkScreee = String;
pub type LinkTarget = String;

pub type Links = BiHashMap<LinkScreee, LinkTarget>;
pub type AppState = State<Arc<Mutex<Links>>>;

pub const DEFAULT_STORE_FILENAME: &str = "./links.json";
pub const DEFAULT_HTTP_ADDR: &str = "0.0.0.0";
pub const DEFAULT_HTTP_PORT: u16 = 3200;
