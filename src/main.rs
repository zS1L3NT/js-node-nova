mod commands;

use {dotenv::dotenv, seahorse::App, std::env::args};

fn main() {
    dotenv().ok();

    let app = App::new("nova")
        .description("A CLI for helping me with various tasks")
        .action(|c| c.help());

    app.run(args().collect::<Vec<String>>());
}
