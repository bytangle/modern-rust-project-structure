pub struct Hashing;

impl Hashing {
  pub fn hash() {
    println!("hashing password...");
  }
}

pub fn hash_password() {
  Hashing::hash();
}