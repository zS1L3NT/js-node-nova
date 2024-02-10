mod commands;
mod models;
mod schema;

pub fn connect_db() -> diesel::SqliteConnection {
    <diesel::SqliteConnection as diesel::Connection>::establish("file:~/nova.db")
        .unwrap_or_else(|_| panic!("Error connecting to ~/nova.db"))
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
