use crate::models::my_model::MyModel;
use chrono::{DateTime, Utc};
use rand::Rng;
use uuid::Uuid;

pub fn test() {
    let mut rng = rand::rng();
    //let model = MyModel {
    //    my_string: Uuid::new_v4().to_string(),
    //    my_i32: rng.random_range(1..=100),
    //    created_at: Utc::now(), 
    //};
    let my_string = Uuid::new_v4().to_string();
    let my_i32 = rng.random_range(1..=100);
    let created_at = Utc::now();
    let model = MyModel::new(&my_string, my_i32, created_at);
    println!("{:?}", model);

}