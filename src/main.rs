mod aes;
mod commands;
mod models;
mod schema;

pub fn create_connection() -> diesel::PgConnection {
    let database_url = option_env!("DATABASE_URL").unwrap();
    <diesel::PgConnection as diesel::Connection>::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    option_env!("AES__ENCRYPTED_KEY").expect("AES__ENCRYPTED_KEY must be set");
    option_env!("AES__NONCE").expect("AES__NONCE must be set");
    option_env!("DATABASE_URL").expect("DATABASE_URL must be set");
    option_env!("PROJECTS_DIR").expect("PROJECTS_DIR must be set");

    let app = seahorse::App::new("nova")
        .description("A CLI for helping me with various tasks")
        .command(commands::configs())
        .command(commands::generate())
        .command(commands::secrets())
        .action(|config| config.help());

    app.run(std::env::args().collect::<Vec<String>>());
}
