use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Request},
    Json,
};

use anyhow::anyhow;

use super::AppError;

pub struct JsonParser<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for JsonParser<T>
where
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => Err(AppError(
                rejection.status(),
                anyhow!(rejection.body_text().to_lowercase()),
            )),
        }
    }
}
