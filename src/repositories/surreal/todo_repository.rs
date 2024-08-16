use surrealdb::{error::Db::Thrown, Error};

static TABLE_NAME: &str = "todo";

use crate::repositories::surreal::models::TodoEntity;

use super::SURREAL_DB;

pub async fn get_all() -> Result<Vec<TodoEntity>, Error> {
  let records = SURREAL_DB.select(TABLE_NAME).await?;
  Ok(records)
}

pub async fn get_by_id(id: String) -> Result<TodoEntity, Error> {
  if let Some(record) = SURREAL_DB.select((TABLE_NAME, id.clone())).await? {
    return Ok(record);
  }

  let error = Error::Db(Thrown(format!("Todo with id {} not found", id)));
  Err(error)
}

pub async fn get_by_title(title: String) -> Result<TodoEntity, Error> {
  if let Some(record) = SURREAL_DB
    .query("SELECT * FROM todo WHERE title = $title")
    .bind(("title", title.clone()))
    .await?
    .take(0)?
  {
    return Ok(record);
  }

  let error = Error::Db(Thrown(format!("Todo with title {} not found", title)));
  Err(error)
}

pub async fn create_todo(content: TodoEntity) -> Result<Vec<TodoEntity>, Error> {
  let record = SURREAL_DB.create(TABLE_NAME).content(content).await?;
  Ok(record)
}

pub async fn update_todo(id: String, content: TodoEntity) -> Result<TodoEntity, Error> {
  let record = SURREAL_DB
    .update((TABLE_NAME, id))
    .content(content)
    .await?
    .unwrap();
  Ok(record)
}

pub async fn delete_todo(id: String) -> Result<TodoEntity, Error> {
  let result = SURREAL_DB.delete((TABLE_NAME, id)).await?.unwrap();
  Ok(result)
}
