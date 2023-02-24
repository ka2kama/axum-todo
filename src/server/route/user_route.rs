use std::sync::Arc;

use axum::body::HttpBody;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};

use crate::usecase::user_usecase::UserUsecase;

type DynUserUsecase = Arc<dyn UserUsecase + Send + Sync>;

pub fn route<S, B>(user_usecase: DynUserUsecase) -> Router<S, B>
where
    B: HttpBody + Send + 'static,
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/:id", get(get_user))
        .with_state(user_usecase)
}

async fn get_user(
    Path(id): Path<String>,
    State(user_usecase): State<DynUserUsecase>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = user_usecase
        .get_user(id)
        .await
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(user))
}
