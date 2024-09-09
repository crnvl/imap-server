use actix_web::{
    get,
    web::{Data, Json, Query},
};
use serde::Deserialize;

use crate::{
    db::{get_all_rows, get_rows_in_range, IterableIP},
    AppState,
};

#[derive(Deserialize)]
pub struct ChunkQuery {
    pub start: i64,
    pub end: i64,
}

#[get("/full_grid")]
pub async fn full_grid(state: Data<AppState>) -> Json<Vec<IterableIP>> {
    println!("[server] full grid requested");
    let data = get_all_rows(state.db.clone()).await;

    Json(data)
}

#[get("/grid_chunk")]
pub async fn grid_chunk(state: Data<AppState>, query: Query<ChunkQuery>) -> Json<Vec<IterableIP>> {
    println!("[server] grid chunk ({} - {}) requested", query.start, query.end);
    let data = get_rows_in_range(state.db.clone(), query.start, query.end).await;

    Json(data)
}
