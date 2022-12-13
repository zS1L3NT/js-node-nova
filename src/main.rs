mod aes;
mod commands;
mod models;
mod schema;

use {
    commands::{config, generate, secrets},
    diesel::{Connection, PgConnection},
    seahorse::App,
    std::env::args,
};

pub fn create_connection() -> PgConnection {
    let database_url = option_env!("DATABASE_URL").unwrap();
    PgConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    option_env!("AES__ENCRYPTED_KEY").expect("AES__ENCRYPTED_KEY must be set");
    option_env!("AES__NONCE").expect("AES__NONCE must be set");
    option_env!("DATABASE_URL").expect("DATABASE_URL must be set");
    option_env!("PROJECTS_DIR").expect("PROJECTS_DIR must be set");

    let app = App::new("nova")
        .description("A CLI for helping me with various tasks")
        .command(config())
        .command(generate())
        .command(secrets())
        .action(|config| config.help());

    app.run(args().collect::<Vec<String>>());
}
