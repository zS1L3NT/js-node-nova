use diesel::update;

use {
    super::super::{
        aes::{decrypt, encrypt, validate},
        create_connection,
        models::Secret,
        schema::secrets,
    },
    dialoguer::{theme::ColorfulTheme, Password},
    diesel::{delete, insert_into, prelude::*},
    prettytable::Table,
    regex::Regex,
    seahorse::Command,
    std::{env::current_dir, fs, path::PathBuf},
};

fn authorize() -> Result<(String, String), String> {
    let regex = Regex::new(r#"^C:\\Projects\\([^\\]*)\\?"#).unwrap();
    if !regex.is_match(current_dir().unwrap().to_str().unwrap()) {
        return Err("Invalid project path".into());
    }

    let directory = current_dir().unwrap();
    let project = match regex.captures(directory.to_str().unwrap()) {
        Some(captures) => captures.get(1).unwrap().as_str(),
        None => {
            return Err("Invalid project path".into());
        }
    };

    let key = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter password: ")
        .interact()
        .unwrap();

    if !validate(&key) {
        Err("Incorrect key".into())
    } else {
        Ok((project.into(), key))
    }
}

fn list() -> Command {
    Command::new("list")
        .description("List all secret filenames for a repository without showing the data")
        .action(|_| {
            let (project, key) = match authorize() {
                Ok((project, key)) => (project, key),
                Err(err) => {
                    println!("{}", err);
                    return;
                }
            };

            let secrets = match secrets::dsl::secrets
                .filter(secrets::project.eq(project))
                .get_results::<Secret>(&mut create_connection())
            {
                Ok(secret) => secret,
                Err(_) => {
                    println!("No secrets found for this project");
                    return;
                }
            };

            let mut table = Table::new();
            table.set_titles(prettytable::row!["Path", "Content Length"]);

            for secret in secrets {
                table.add_row(prettytable::row![
                    secret.path,
                    decrypt(&secret.content, &key).unwrap().len()
                ]);
            }

            table.printstd();
        })
}

fn clone() -> Command {
    Command::new("clone")
        .description("Clone the repository secrets to their original locations")
        .action(|_| {
            let (project, key) = match authorize() {
                Ok((project, key)) => (project, key),
                Err(err) => {
                    println!("{}", err);
                    return;
                }
            };

            let secrets = match secrets::dsl::secrets
                .filter(secrets::project.eq(project))
                .get_results::<Secret>(&mut create_connection())
            {
                Ok(secret) => secret,
                Err(_) => {
                    println!("No secrets found for this project");
                    return;
                }
            };

            for secret in secrets {
                match fs::write(
                    PathBuf::from(&secret.path),
                    decrypt(&secret.content, &key).unwrap(),
                ) {
                    Ok(_) => println!("Wrote to file: {}", secret.path),
                    Err(err) => {
                        println!("Unable to write file: {}", secret.path);
                        println!("Error: {}", err);
                    }
                }
            }
        })
}

fn set() -> Command {
    Command::new("set")
        .description("Set a repository secret, update if it already exists")
        .action(|context| {
            let (project, key) = match authorize() {
                Ok((project, key)) => (project, key),
                Err(err) => {
                    println!("{}", err);
                    return;
                }
            };

            let path = match context.args.first() {
                Some(path) => path.to_string(),
                None => {
                    println!("No path provided");
                    return;
                }
            };

            let content = match fs::read_to_string(&path) {
                Ok(content) => content,
                Err(_) => {
                    println!("Unable to read file: {}", path);
                    return;
                }
            };

            let secret = Secret {
                project,
                path: path.clone(),
                content: encrypt(&content, &key).unwrap(),
            };

            let upsert = match secrets::dsl::secrets
                .filter(secrets::project.eq(&secret.project))
                .filter(secrets::path.eq(&path))
                .first::<Secret>(&mut create_connection())
            {
                Ok(_) => update(secrets::dsl::secrets)
                    .filter(secrets::project.eq(&secret.project))
                    .filter(secrets::path.eq(&path))
                    .set(secrets::content.eq(&secret.content))
                    .execute(&mut create_connection()),
                Err(_) => insert_into(secrets::dsl::secrets)
                    .values(&secret)
                    .execute(&mut create_connection()),
            };

            match upsert {
                Ok(_) => {
                    println!("Secret stored: {}", path);
                }
                Err(err) => {
                    println!("Unable to store secret: {}", path);
                    println!("Error: {}", err);
                }
            }
        })
}

fn remove() -> Command {
    Command::new("remove")
        .description("Remove a repository secret")
        .action(|context| {
            let (project, _) = match authorize() {
                Ok((project, key)) => (project, key),
                Err(err) => {
                    println!("{}", err);
                    return;
                }
            };

            let path = match context.args.first() {
                Some(path) => path.to_string(),
                None => {
                    println!("No path provided");
                    return;
                }
            };

            match delete(secrets::dsl::secrets)
                .filter(secrets::project.eq(project))
                .filter(secrets::path.eq(&path))
                .execute(&mut create_connection())
            {
                Ok(deleted) => {
                    if deleted == 0 {
                        println!("No secret found stored with this path");
                    } else {
                        println!("Secret removed: {}", path);
                    }
                }
                Err(err) => {
                    println!("No secret found stored with this path");
                    println!("Error: {}", err);
                }
            };
        })
}

pub fn secrets() -> Command {
    Command::new("secrets")
        .description("Manage secrets for different repositories")
        .command(list())
        .command(clone())
        .command(set())
        .command(remove())
        .action(|context| context.help())
}
