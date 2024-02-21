mod commands;
mod models;
mod output;
mod schema;

pub fn connect_db() -> diesel::SqliteConnection {
    if sudo::escalate_if_needed().is_err() {
        panic!("Sudo permission required to access secrets");
    }

    <diesel::SqliteConnection as diesel::Connection>::establish("file:/Users/mac/nova.db")
        .unwrap_or_else(|_| panic!("Error connecting to /Users/mac/nova.db"))
}

fn main() {
    let app = seahorse::App::new("nova")
        .description("A CLI for helping me with various tasks")
        .command(commands::configs())
        .command(commands::generate())
        .command(commands::secrets())
        .command(commands::setup())
        .action(|config| config.help());

    app.run(std::env::args().collect::<Vec<String>>());
}
