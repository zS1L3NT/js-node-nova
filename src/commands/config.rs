use {
    super::super::{create_connection, models::Config, schema::configs},
    diesel::prelude::*,
    prettytable::Table,
    seahorse::Command,
    std::{fs, path::PathBuf},
};

fn clone() -> Command {
    Command::new("clone")
        .description("Clone project configuration file(s) to the current working directory")
        .action(|context| {
            for shorthand in &context.args {
                let config = match configs::dsl::configs
                    .filter(configs::shorthand.eq(shorthand))
                    .first::<Config>(&mut create_connection())
                {
                    Ok(config) => config,
                    Err(_) => {
                        println!("Unknown config shorthand: {}", shorthand);
                        continue;
                    }
                };

                match fs::write(PathBuf::from(&config.filename), config.content) {
                    Ok(_) => println!("Wrote to file: {}", config.filename),
                    Err(err) => {
                        println!("Unable to write file: {}", config.filename);
                        println!("Error: {}", err);
                    }
                }
            }
        })
}

fn list() -> Command {
    Command::new("list")
        .description("List all project configuration file(s) and their shorthands")
        .action(|_| {
            let configs = configs::dsl::configs
                .load::<Config>(&mut create_connection())
                .unwrap();

            let mut table = Table::new();
			table.set_titles(prettytable::row!["Shorthand", "Filename"]);

            for config in configs {
				table.add_row(prettytable::row![config.shorthand, config.filename]);
            }

			table.printstd();
        })
}

pub fn config() -> Command {
    Command::new("config")
        .description("Manage reusable project configuration files")
        .command(clone())
        .command(list())
        .action(|context| context.help())
}
