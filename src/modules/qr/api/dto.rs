use serde::{Deserialize, Serialize};

use crate::modules::qr::Qr;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QrDto {
    pub id: String,
    pub have_owner: bool,
}

impl From<Qr> for QrDto {
    fn from(qr: Qr) -> Self {
        QrDto {
            id: qr.id,
            have_owner: qr.email.is_some(),
        }
    }
}
