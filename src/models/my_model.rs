use crate::my_traits::repo::RandomRepo;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct MyModel {
    pub my_string: String,
    pub my_i32: i32,
    pub created_at: DateTime<Utc>,
}

impl MyModel {
    pub fn new(my_string: &str, my_i32: i32, created_at: DateTime<Utc>) -> Self {
        let model = MyModel {
            my_string: my_string.to_string(),
            my_i32: my_i32,
            created_at: created_at,
        };
        model
    }
    pub async fn save(&self, repo: &impl RandomRepo) -> Result<(), String> { //&self is self:&MyModel
        repo.save(self).await?;
        Ok(())
    }
}
