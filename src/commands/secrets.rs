use {
    crate::{models::Secret, schema::secrets},
    diesel::prelude::*,
};

struct Location {
    project: String,
    folder: Option<String>,
}

fn authorize() -> Result<Location, String> {
    let mut location = Location {
        project: String::new(),
        folder: None,
    };

    match regex::Regex::new(r#"Projects/([^/]*)/?(.+)?"#)
        .unwrap()
        .captures(
            &std::env::current_dir()
                .unwrap()
                .to_str()
                .unwrap()
                .replace('\\', "/"),
        ) {
        Some(captures) => {
            location.project = captures.get(1).unwrap().as_str().into();
            location.folder = captures.get(2).map(|m| m.as_str().to_string());
        }
        None => {
            return Err("Invalid project path".into());
        }
    };

    match sudo::escalate_if_needed() {
        Ok(_) => Ok(location),
        Err(_) => Err("Sudo permission required to access secrets".into()),
    }
}

fn list() -> seahorse::Command {
    seahorse::Command::new("list")
        .description("List all secret filenames for a repository without showing the data")
        .usage("nova secrets list")
        .action(|_| {
            let location = match authorize() {
                Ok(location) => location,
                Err(err) => {
                    println!("{}", err);
                    return;
                }
            };

            let secrets = match secrets::dsl::secrets
                .filter(secrets::project.eq(&location.project))
                .get_results::<Secret>(&mut crate::connect_db())
            {
                Ok(secret) => secret,
                Err(_) => {
                    println!("No secrets found for this project");
                    return;
                }
            };

            let mut table = prettytable::Table::new();
            table.set_titles(prettytable::row!["Path", "Content Length"]);

            for secret in secrets {
                table.add_row(prettytable::row![secret.path, secret.content.len()]);
            }

            table.printstd();
        })
}

fn clone() -> seahorse::Command {
    seahorse::Command::new("clone")
        .description("Clone the repository secrets to their original locations")
        .usage("nova secrets clone")
        .action(|_| {
            let location = match authorize() {
                Ok(location) => location,
                Err(err) => {
                    println!("{}", err);
                    return;
                }
            };

            let secrets = match secrets::dsl::secrets
                .filter(secrets::project.eq(&location.project))
                .get_results::<Secret>(&mut crate::connect_db())
            {
                Ok(secret) => secret,
                Err(_) => {
                    println!("No secrets found for this project");
                    return;
                }
            };

            for secret in secrets {
                let absolute_path = std::path::PathBuf::from("/Users/mac/Projects")
                    .join(&secret.project)
                    .join(&secret.path);
                match std::fs::write(&absolute_path, &secret.content) {
                    Ok(_) => println!("Wrote to file: {}", &secret.path),
                    Err(err) => {
                        println!("Unable to write file: {}", &secret.path);
                        println!("Error: {}", err);
                    }
                }
            }
        })
}

fn check() -> seahorse::Command {
    seahorse::Command::new("check")
        .description("Check if the secrets are still the same as that in the database")
        .usage("nova secrets check")
        .action(|_| {
            let location = match authorize() {
                Ok(location) => location,
                Err(err) => {
                    println!("{}", err);
                    return;
                }
            };

            let secrets = match secrets::dsl::secrets
                .filter(secrets::project.eq(&location.project))
                .get_results::<Secret>(&mut crate::connect_db())
            {
                Ok(secret) => secret,
                Err(_) => {
                    println!("No secrets found for this project");
                    return;
                }
            };

            for secret in secrets {
                let absolute_path = std::path::PathBuf::from("/Users/mac/Projects")
                    .join(&secret.project)
                    .join(&secret.path);
                match std::fs::read_to_string(&absolute_path) {
                    Ok(content) => {
                        if content == secret.content {
                            println!("Identical secret: {}", &secret.path);
                        } else {
                            println!("Non-identical secret: {}", &secret.path);
                        }
                    }
                    Err(_) => {
                        println!("Non-existent secret: {}", &secret.path);
                    }
                }
            }
        })
}

fn set() -> seahorse::Command {
    seahorse::Command::new("set")
        .description("Set a repository secret, update if it already exists")
        .usage("nova secrets set [path/to/config]")
        .action(|context| {
            let location = match authorize() {
                Ok(location) => location,
                Err(err) => {
                    println!("{}", err);
                    return;
                }
            };

            let cwd_relative_path = match context.args.first() {
                Some(path) => path.to_string().replace('\\', "/"),
                None => {
                    println!("No path provided");
                    return;
                }
            };

            let content = match std::fs::read_to_string(&cwd_relative_path) {
                Ok(content) => content,
                Err(_) => {
                    println!("Unable to read file: {}", cwd_relative_path);
                    return;
                }
            };

            let secret = Secret {
                project: location.project,
                path: std::path::PathBuf::from(&location.folder.unwrap_or_default())
                    .join(&cwd_relative_path)
                    .to_str()
                    .unwrap()
                    .into(),
                content: content,
            };

            let upsert = match secrets::dsl::secrets
                .filter(secrets::project.eq(&secret.project))
                .filter(secrets::path.eq(&secret.path))
                .first::<Secret>(&mut crate::connect_db())
            {
                Ok(_) => diesel::update(secrets::dsl::secrets)
                    .filter(secrets::project.eq(&secret.project))
                    .filter(secrets::path.eq(&secret.path))
                    .set(secrets::content.eq(&secret.content))
                    .execute(&mut crate::connect_db()),
                Err(_) => diesel::insert_into(secrets::dsl::secrets)
                    .values(&secret)
                    .execute(&mut crate::connect_db()),
            };

            match upsert {
                Ok(_) => {
                    println!("Secret stored: {}", &secret.path);
                }
                Err(err) => {
                    println!("Unable to store secret: {}", &secret.path);
                    println!("Error: {}", err);
                }
            }
        })
}

fn remove() -> seahorse::Command {
    seahorse::Command::new("remove")
        .description("Remove a repository secret")
        .usage("nova secrts remove [path/to/config]")
        .action(|context| {
            let location = match authorize() {
                Ok(location) => location,
                Err(err) => {
                    println!("{}", err);
                    return;
                }
            };

            let cwd_relative_path = match context.args.first() {
                Some(path) => path.to_string().replace('\\', "/"),
                None => {
                    println!("No path provided");
                    return;
                }
            };

            let project_relative_path: String =
                std::path::PathBuf::from(&location.folder.unwrap_or_default())
                    .join(cwd_relative_path)
                    .to_str()
                    .unwrap()
                    .into();
            match diesel::delete(secrets::dsl::secrets)
                .filter(secrets::project.eq(&location.project))
                .filter(secrets::path.eq(&project_relative_path))
                .execute(&mut crate::connect_db())
            {
                Ok(deleted) => {
                    if deleted == 0 {
                        println!("No secret found stored with this path");
                    } else {
                        println!("Secret removed: {}", project_relative_path);
                    }
                }
                Err(err) => {
                    println!("No secret found stored with this path");
                    println!("Error: {}", err);
                }
            };
        })
}

pub fn secrets() -> seahorse::Command {
    seahorse::Command::new("secrets")
        .description("Manage secrets for different repositories")
        .command(list())
        .command(clone())
        .command(check())
        .command(set())
        .command(remove())
        .action(|context| context.help())
}
