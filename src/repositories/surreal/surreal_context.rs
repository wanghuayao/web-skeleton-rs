use once_cell::sync::Lazy;
use surrealdb::{
  engine::remote::ws::{Client, Ws},
  opt::auth::Root,
  Result, Surreal,
};

pub static SURREAL_DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

pub async fn connect_surreal_db() -> Result<()> {
  let _ = SURREAL_DB.connect::<Ws>("localhost:8000").await?;
  let _ = SURREAL_DB
    .signin(Root {
      username: "root",
      password: "root",
    })
    .await;
  let _ = SURREAL_DB.use_ns("todo").use_db("todo").await?;
  Ok(())
}
