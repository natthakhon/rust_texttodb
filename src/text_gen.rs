use crate::{models::my_model::MyModel, my_traits::repo::RandomRepo};
use crate::data::pgsql::PgSqlRandomRepo;
use chrono::{DateTime, Utc};
use rand::Rng;
use uuid::Uuid;

pub async fn test() -> Result<(), String>{
    let mut rng = rand::rng();
    let my_string = Uuid::new_v4().to_string();
    let my_i32 = rng.random_range(1..=100);
    let created_at = Utc::now();
    let model = MyModel::new(&my_string, my_i32, created_at);
    println!("{:?}", model);
    let repo=PgSqlRandomRepo::new();
    repo.save(&model).await?;
    Ok(())
}