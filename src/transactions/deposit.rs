pub struct Deposit;

impl Deposit {
  pub fn make() {
    println!("processing deposits...");
  }
}

pub fn make_deposit() {
  Deposit::make();
}