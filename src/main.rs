mod commands;
#[allow(warnings, unused)]
mod prisma;

pub async fn connect_db() -> prisma::PrismaClient {
    prisma::PrismaClient::_builder().build().await.unwrap()
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
