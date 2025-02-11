pub struct Auth;

impl Auth {
  pub fn login() {
    println!("User logged in successfully!");
  }
}

pub fn login() {
  Auth::login();
}