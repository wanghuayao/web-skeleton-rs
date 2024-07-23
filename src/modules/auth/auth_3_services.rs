use super::auth_1_models::User;

pub fn login_service(user: &User) -> Result<(), String> {
  println!("login_service {:?}", user);

  Ok(())
}
