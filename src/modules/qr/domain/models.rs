use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::option::Option;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Qr {
    pub id: String,                             // UUID stored as a String
    pub email: Option<String>,                  // Optional email
    pub password: String,                       // Fixed length password 10
    pub creation_date: DateTime<Utc>,           // Timestamp with timezone
    pub redemption_date: Option<DateTime<Utc>>, // Optional Date of redemption
}
impl Qr {
    pub fn new() -> Qr {
        Qr {
            id: uuid::Uuid::new_v4().to_string(),
            email: None,
            password: Qr::generate_password(),
            creation_date: Utc::now(),
            redemption_date: None,
        }
    }

    pub fn delete_holder(&mut self) {
        self.email = None;
        self.password = Qr::generate_password();
    }

    pub fn update_password(&mut self) {
        self.password = Qr::generate_password();
    }

    fn generate_password() -> String {
        //password should be 10 chars, with mayus,numbers,munus
        let password = (0..10)
            .map(|_| {
                let choice = rand::random::<u8>() % 3;
                match choice {
                    0 => (rand::random::<u8>() % 26 + 65) as char,
                    1 => (rand::random::<u8>() % 26 + 97) as char,
                    2 => (rand::random::<u8>() % 10 + 48) as char,
                    _ => panic!("Invalid choice"), //this will never panic
                }
            })
            .collect::<String>();

        password
    }
}
