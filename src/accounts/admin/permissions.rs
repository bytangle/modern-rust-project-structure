pub struct Permissions;

impl Permissions {
  pub fn check() {
    println!("Checking admin permissions...");
  }
}

pub fn check_permissions() {
  Permissions::check();
}