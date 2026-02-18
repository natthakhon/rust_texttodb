use crate::models::my_model::MyModel;
use crate::my_traits::repo::RandomRepo;
use sqlx::postgres::PgPoolOptions;
use chrono::Utc;
pub struct PgSqlRandomRepo {}

impl PgSqlRandomRepo {
    pub fn new() -> Self {
        Self {}
    }
}
#[async_trait::async_trait]
impl RandomRepo for PgSqlRandomRepo {
    async fn save(&self, my_model: &MyModel) -> Result<(), String> {
        let db_url = "postgres://postgres:postgres@localhost:5432/rust";
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
            .map_err(|e| e.to_string())?;

        sqlx::query(r#"INSERT INTO test (mystring, myint, mydate) VALUES ($1, $2, $3)"#)
        .bind(&my_model.my_string)
        .bind(my_model.my_i32)
        .bind(my_model.created_at)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

        println!("{:?}", my_model);
        Ok(())
    }
}
