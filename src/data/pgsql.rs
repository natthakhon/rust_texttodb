use crate::models::my_model::MyModel;
use crate::my_traits::repo::RandomRepo;
pub struct PgSqlRandomRepo {}

impl PgSqlRandomRepo {
    pub fn new() -> Self {
        Self {}
    }
}
#[async_trait::async_trait]
impl RandomRepo for PgSqlRandomRepo {
    async fn save(&self, my_model: &MyModel) -> Result<(), String> {
        println!("whats up!");
        Ok(())
    }
}
