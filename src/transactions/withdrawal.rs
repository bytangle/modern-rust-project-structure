pub struct Withdrawal;

impl Withdrawal {
  pub fn make() {
    println!("Processing withdrawal...");
  }
}

pub fn make_withdrawal() {
  Withdrawal::make();
}