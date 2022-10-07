mod commands;
mod models;
mod schema;

use {
    commands::config,
    diesel::{Connection, PgConnection},
    dotenv::dotenv,
    seahorse::App,
    std::env::{self, args},
};

pub fn create_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    let app = App::new("nova")
        .description("A CLI for helping me with various tasks")
        .command(config())
        .action(|config| config.help());

    app.run(args().collect::<Vec<String>>());
}
