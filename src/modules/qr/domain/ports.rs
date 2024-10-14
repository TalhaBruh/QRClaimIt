use async_trait::async_trait;

use crate::error::AppError;

use super::Qr;

#[async_trait]
pub trait Repository: Send + Sync {
    async fn save(&self, data: &Qr) -> Result<(), AppError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Qr>, AppError>;
    async fn update(&self, data: &Qr) -> Result<(), AppError>;
}
