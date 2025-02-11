use crate::accounts::admin::permissions;

pub struct Auth;

impl Auth {
  pub fn login() {
    println!("User logged in successfully!");
  }
}

pub fn login() {
  permissions::check_permissions();
  Auth::login();
}