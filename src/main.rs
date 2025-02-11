mod accounts;
mod transactions;
mod security;
mod shared;

fn main() {
	// accounts module
	accounts::user::profile::show_profile();
	accounts::user::settings::update_settings();
	accounts::admin::permissions::check_permissions();
	accounts::admin::reports::generate_report();

	// transactions module
	transactions::deposit::make_deposit();
	transactions::withdrawal::make_withdrawal();

	// security module
	security::auth::login();
	security::encryption::encrypt_data();
	security::hashing::hash_password();

	// shared module
	shared::utils::say_hi::say_hi();
}
