pub struct Report;

impl Report {
  pub fn generate() {
    println!("Generating admin report...");
  }
}

pub fn generate_report() {
  Report::generate();
}