use crate::error::AppError;
use crate::modules::qr::ports::Repository;
use crate::modules::qr::Qr;
use crate::utils::db::PostgresRepository;
use async_trait::async_trait;

#[async_trait]
impl Repository for PostgresRepository {
    async fn save(&self, data: &Qr) -> Result<(), AppError> {
        let query = r#"
            INSERT INTO qr (id, email, password, creation_date, redemption_date)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO NOTHING;
        "#;
        sqlx::query(query)
            .bind(&data.id)
            .bind(&data.email)
            .bind(&data.password)
            .bind(&data.creation_date)
            .bind(&data.redemption_date)
            .execute(&*self.pg_pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Qr>, AppError> {
        let query = r#"
            SELECT id, email, password, creation_date, redemption_date
            FROM qr
            WHERE id = $1;
        "#;
        match sqlx::query_as::<_, Qr>(query)
            .bind(id)
            .fetch_optional(&*self.pg_pool)
            .await
        {
            Ok(result) => Ok(result),
            Err(e) => Err(AppError::DatabaseError(e.to_string())),
        }
    }

    async fn update(&self, data: &Qr) -> Result<(), AppError> {
        let query = r#"
            UPDATE qr
            SET email = $2, password = $3, redemption_date = $4
            WHERE id = $1;
        "#;
        sqlx::query(query)
            .bind(&data.id)
            .bind(&data.email)
            .bind(&data.password)
            .bind(&data.redemption_date)
            .execute(&*self.pg_pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }
}
