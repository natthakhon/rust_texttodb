pub mod text_gen;
pub mod models;
pub mod my_traits;
pub mod data;

#[tokio::main]
async fn main()-> Result<(), String>{
    text_gen::test().await?;
    Ok(())
}
