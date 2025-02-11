pub struct Profiles;

impl Profiles {
  pub fn show() {
    println!("Showing user profile...");
  }
}

pub fn show_profile() {
  Profiles::show();
}