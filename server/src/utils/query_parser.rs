use axum::{
    async_trait,
    extract::{rejection::QueryRejection, FromRequestParts, Query},
    http::request::Parts,
};

use anyhow::anyhow;

use super::AppError;

pub struct QueryParser<T>(pub T);

#[async_trait]
impl<T, S> FromRequestParts<S> for QueryParser<T>
where
    Query<T>: FromRequestParts<S, Rejection = QueryRejection>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match Query::<T>::from_request_parts(parts, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => Err(AppError(
                rejection.status(),
                anyhow!(rejection.body_text().to_lowercase()),
            )),
        }
    }
}
