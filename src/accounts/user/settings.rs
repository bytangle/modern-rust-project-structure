struct Settings;

impl Settings {
  pub fn update() {
    println!("Updating user settings...");
  }
}

pub fn update_settings() {
  Settings::update();
}