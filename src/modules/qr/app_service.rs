use std::sync::Arc;

use crate::error::AppError;

use super::{ports::Repository, Qr};

pub struct Service {
    repository: Arc<dyn Repository>,
}
impl Service {
    pub fn new(repository: Arc<dyn Repository>) -> Self {
        Self { repository }
    }
}

impl Service {
    pub async fn create_qr(&self) -> Result<Qr, AppError> {
        let new_qr = Qr::new();
        self.repository.save(&new_qr).await?;
        Ok(new_qr)
    }

    pub async fn get_qr(&self, id: &str) -> Result<Qr, AppError> {
        match self.repository.find_by_id(id).await {
            Ok(qr) => match qr {
                Some(qr) => Ok(qr),
                None => Err(AppError::NotFound("".into())),
            },
            Err(e) => Err(e),
        }
    }

    pub async fn remove_holder(
        &self,
        qr_id: &str,
        holder_email: &str,
        password: &str,
    ) -> Result<(), AppError> {
        let mut qr = match self.repository.find_by_id(qr_id).await? {
            Some(qr) => qr,
            None => return Err(AppError::NotFound("".into())),
        };

        match qr.email {
            Some(ref email) => {
                if email != holder_email {
                    return Err(AppError::Unauthorized("".into()));
                }
                if password != qr.password {
                    return Err(AppError::Unauthorized("".into()));
                }

                qr.delete_holder();
                return self.repository.update(&qr).await;
            }
            _ => return Err(AppError::BadRequest("".into())),
        }
    }

    pub async fn new_holder(&self, id: &str, email: &str) -> Result<(), AppError> {
        let mut qr = match self.repository.find_by_id(id).await? {
            Some(qr) => qr,
            None => return Err(AppError::NotFound("".into())),
        };

        if qr.email.is_some() {
            return Err(AppError::Unauthorized("Qr have user".into()));
        }

        qr.email = Some(email.into());
        return self.repository.update(&qr).await;
    }
}
