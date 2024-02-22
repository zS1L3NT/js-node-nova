use {
    crate::{error, models::Config, schema::configs, success, warn},
    diesel::prelude::*,
};

fn list() -> seahorse::Command {
    seahorse::Command::new("list")
        .description("List all project configuration file(s) and their shorthands")
        .usage("nova configs list")
        .action(|_| {
            let configs = configs::dsl::configs
                .load::<Config>(&mut crate::connect_db())
                .unwrap();

            let mut table = prettytable::Table::new();
            table.set_titles(prettytable::row!["Shorthand", "Filename", "Content Length"]);

            for config in configs {
                table.add_row(prettytable::row![
                    config.shorthand,
                    config.filename,
                    config.content.len()
                ]);
            }

            table.printstd();
        })
}

fn clone() -> seahorse::Command {
    seahorse::Command::new("clone")
        .description("Clone project configuration file(s) to the current working directory")
        .usage("nova configs clone [...shorthands]")
        .action(|context| {
            for shorthand in &context.args {
                let config = match configs::dsl::configs
                    .filter(configs::shorthand.eq(shorthand))
                    .first::<Config>(&mut crate::connect_db())
                {
                    Ok(config) => config,
                    Err(_) => {
                        error!("Unknown config shorthand", shorthand);
                        continue;
                    }
                };

                if let Err(err) =
                    std::fs::write(std::path::PathBuf::from(&config.filename), config.content)
                {
                    error!("Unable to write to file", config.filename; err);
                    return;
                }

                if let Err(err) = std::os::unix::fs::chown(
                    std::path::PathBuf::from(&config.filename),
                    Some(501),
                    Some(20),
                ) {
                    error!("Unable to change file owner", config.filename; err);
                    return;
                }

                success!("Cloned file", config.filename);
            }

            if context.args.is_empty() {
                error!("Please provide some shorthands to clone");
            }
        })
}

fn vim() -> seahorse::Command {
    seahorse::Command::new("vim")
        .description("View a project configuration file in Vim")
        .usage("nova configs vim [shorthand]")
        .action(|context| {
            let shorthand = match context.args.first() {
                Some(shorthand) => shorthand,
                None => {
                    error!("Please provide a filename to edit");
                    return;
                }
            };

            let config = match configs::dsl::configs
                .filter(configs::shorthand.eq(shorthand))
                .first::<Config>(&mut crate::connect_db())
            {
                Ok(config) => config,
                Err(_) => {
                    error!("Unknown config shorthand", shorthand);
                    return;
                }
            };

            let path = std::path::PathBuf::from(format!("/Users/mac/{}.temp", &config.filename));
            if let Err(err) = std::fs::write(&path, &config.content) {
                error!("Unable to write to temp file", config.filename; err);
                return;
            }

            let mut child = match std::process::Command::new("/opt/homebrew/bin/nvim")
                .arg(&path.to_str().unwrap())
                .spawn()
            {
                Ok(child) => child,
                Err(err) => {
                    error!("Unable to run nvim"; err);
                    return;
                }
            };

            match child.wait() {
                Ok(_) => {}
                Err(err) => {
                    error!("Nvim returned a non-zero status"; err);
                    return;
                }
            }

            let content = match std::fs::read_to_string(&path) {
                Ok(content) => content,
                Err(err) => {
                    error!("Unable to read from temp file", config.filename; err);
                    return;
                }
            };

            if let Err(err) = std::fs::remove_file(path) {
                error!("Unable to remove temp file", config.filename; err);
                return;
            }

            if &content == &config.content {
                warn!("No changes made to file", config.filename);
                return;
            }

            match diesel::update(configs::dsl::configs)
                .filter(configs::shorthand.eq(&shorthand))
                .set(configs::content.eq(&content))
                .execute(&mut crate::connect_db())
            {
                Ok(_) => {
                    success!("Updated config", &config.filename);
                }
                Err(err) => {
                    error!("Unable to update config", &config.filename; err);
                }
            }
        })
}

fn add() -> seahorse::Command {
    seahorse::Command::new("add")
        .description("Add a new configuration file, uses file content if the file exists")
        .usage("nova configs add [shorthand] [filename]")
        .action(|context| {
            let shorthand = match context.args.first() {
                Some(shorthand) => shorthand,
                None => {
                    error!("Please provide a shorthand, then a filename");
                    return;
                }
            };

            let filename = match context.args.get(1) {
                Some(filename) => filename,
                None => {
                    error!("Please provide a filename");
                    return;
                }
            };

            let content = match std::fs::read_to_string(std::path::PathBuf::from(&filename)) {
                Ok(content) => content,
                Err(_) => {
                    warn!("Could not read file data", filename);
                    String::new()
                }
            };

            match configs::dsl::configs
                .filter(configs::shorthand.eq(shorthand))
                .count()
                .get_result::<i64>(&mut crate::connect_db())
            {
                Ok(configs) => {
                    if configs != 0 {
                        error!("Shorthand already exists");
                        return;
                    }
                }
                Err(err) => {
                    error!("Unable to fetch configs"; err);
                    return;
                }
            };

            match configs::dsl::configs
                .filter(configs::filename.eq(filename))
                .count()
                .get_result::<i64>(&mut crate::connect_db())
            {
                Ok(configs) => {
                    if configs != 0 {
                        error!("Filename already exists");
                        return;
                    }
                }
                Err(err) => {
                    error!("Unable to fetch configs"; err);
                    return;
                }
            };

            let config = Config {
                filename: filename.to_string(),
                shorthand: shorthand.to_string(),
                content,
            };

            match diesel::insert_into(configs::dsl::configs)
                .values(&config)
                .execute(&mut crate::connect_db())
            {
                Ok(_) => {
                    success!(format!(
                        "Added config \"{shorthand}\" which expands to \"{filename}\""
                    ))
                }
                Err(err) => {
                    error!("Unable to store new config", shorthand; err);
                }
            }
        })
}

fn remove() -> seahorse::Command {
    seahorse::Command::new("remove")
        .description("Remove a configuration file")
        .usage("nova configs remove [shorthand]")
        .action(|context| {
            let shorthand = match context.args.first() {
                Some(shorthand) => shorthand,
                None => {
                    error!("Please provide a shorthand");
                    return;
                }
            };

            match diesel::delete(configs::dsl::configs)
                .filter(configs::shorthand.eq(&shorthand))
                .execute(&mut crate::connect_db())
            {
                Ok(deleted) => {
                    if deleted == 0 {
                        error!("Unknown config shorthand", shorthand);
                    } else {
                        success!("Removed config", shorthand);
                    }
                }
                Err(err) => {
                    error!("Unable to delete config", shorthand; err);
                }
            };
        })
}

pub fn configs() -> seahorse::Command {
    seahorse::Command::new("configs")
        .description("Manage reusable project configuration files")
        .command(list())
        .command(clone())
        .command(vim())
        .command(add())
        .command(remove())
        .action(|context| context.help())
}
