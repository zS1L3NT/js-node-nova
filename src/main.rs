mod aes;
mod commands;
mod models;
mod schema;

use {
    commands::{config, generate, secret},
    diesel::{Connection, PgConnection},
    seahorse::App,
    std::env::{self, args},
};

pub fn create_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    dotenv::from_filename("C:/Projects/rs-nova/.env").expect("Cannot load environment variables");

    env::var("AES__ENCRYPTED_KEY").expect("AES__ENCRYPTED_KEY must be set");
    env::var("AES__NONCE").expect("AES__NONCE must be set");
    env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let app = App::new("nova")
        .description("A CLI for helping me with various tasks")
        .command(config())
        .command(generate())
        .command(secret())
        .action(|config| config.help());

    app.run(args().collect::<Vec<String>>());
}
