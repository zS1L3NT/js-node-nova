use {
    crate::{models::Config, schema::configs},
    diesel::prelude::*,
};

fn clone() -> seahorse::Command {
    seahorse::Command::new("clone")
        .description("Clone project configuration file(s) to the current working directory")
        .action(|context| {
            for shorthand in &context.args {
                let config = match configs::dsl::configs
                    .filter(configs::shorthand.eq(shorthand))
                    .first::<Config>(&mut crate::create_connection())
                {
                    Ok(config) => config,
                    Err(_) => {
                        println!("Unknown config shorthand: {}", shorthand);
                        continue;
                    }
                };

                match std::fs::write(std::path::PathBuf::from(&config.filename), config.content) {
                    Ok(_) => println!("Wrote to file: {}", config.filename),
                    Err(err) => {
                        println!("Unable to write file: {}", config.filename);
                        println!("Error: {}", err);
                    }
                }
            }
        })
}

fn list() -> seahorse::Command {
    seahorse::Command::new("list")
        .description("List all project configuration file(s) and their shorthands")
        .action(|_| {
            let configs = configs::dsl::configs
                .load::<Config>(&mut crate::create_connection())
                .unwrap();

            let mut table = prettytable::Table::new();
            table.set_titles(prettytable::row!["Shorthand", "Filename"]);

            for config in configs {
                table.add_row(prettytable::row![config.shorthand, config.filename]);
            }

            table.printstd();
        })
}

pub fn config() -> seahorse::Command {
    seahorse::Command::new("config")
        .description("Manage reusable project configuration files")
        .command(clone())
        .command(list())
        .action(|context| context.help())
}
