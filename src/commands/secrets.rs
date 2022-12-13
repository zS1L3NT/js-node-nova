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

struct AuthData {
    project: String,
    folder: Option<String>,
    key: String,
}

fn authorize() -> Result<AuthData, String> {
    let mut response = AuthData {
        project: String::new(),
        folder: None,
        key: String::new(),
    };

    match Regex::new(r#"Projects/([^/]*)/?(.+)?"#)
        .unwrap()
        .captures(&current_dir().unwrap().to_str().unwrap().replace('\\', "/"))
    {
        Some(captures) => {
            response.project = captures.get(1).unwrap().as_str().into();
            response.folder = captures.get(2).map(|m| m.as_str().to_string());
        }
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
        response.key = key;
        Ok(response)
    }
}

fn list() -> Command {
    Command::new("list")
        .description("List all secret filenames for a repository without showing the data")
        .action(|_| {
            let auth = match authorize() {
                Ok(auth) => auth,
                Err(err) => {
                    println!("{}", err);
                    return;
                }
            };

            let secrets = match secrets::dsl::secrets
                .filter(secrets::project.eq(&auth.project))
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
                    decrypt(&secret.content, &auth.key).unwrap().len()
                ]);
            }

            table.printstd();
        })
}

fn clone() -> Command {
    Command::new("clone")
        .description("Clone the repository secrets to their original locations")
        .action(|_| {
            let auth = match authorize() {
                Ok(auth) => auth,
                Err(err) => {
                    println!("{}", err);
                    return;
                }
            };

            let secrets = match secrets::dsl::secrets
                .filter(secrets::project.eq(&auth.project))
                .get_results::<Secret>(&mut create_connection())
            {
                Ok(secret) => secret,
                Err(_) => {
                    println!("No secrets found for this project");
                    return;
                }
            };

            for secret in secrets {
                let absolute_path = PathBuf::from(option_env!("PROJECTS_DIR").unwrap())
                    .join(&secret.project)
                    .join(&secret.path);
                match fs::write(&absolute_path, decrypt(&secret.content, &auth.key).unwrap()) {
                    Ok(_) => println!("Wrote to file: {}", &secret.path),
                    Err(err) => {
                        println!("Unable to write file: {}", &secret.path);
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
            let auth = match authorize() {
                Ok(auth) => auth,
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

            let content = match fs::read_to_string(&cwd_relative_path) {
                Ok(content) => content,
                Err(_) => {
                    println!("Unable to read file: {}", cwd_relative_path);
                    return;
                }
            };

            let secret = Secret {
                project: auth.project,
                path: PathBuf::from(&auth.folder.unwrap_or_default())
                    .join(&cwd_relative_path)
                    .to_str()
                    .unwrap()
                    .into(),
                content: encrypt(&content, &auth.key).unwrap(),
            };

            let upsert = match secrets::dsl::secrets
                .filter(secrets::project.eq(&secret.project))
                .filter(secrets::path.eq(&secret.path))
                .first::<Secret>(&mut create_connection())
            {
                Ok(_) => update(secrets::dsl::secrets)
                    .filter(secrets::project.eq(&secret.project))
                    .filter(secrets::path.eq(&secret.path))
                    .set(secrets::content.eq(&secret.content))
                    .execute(&mut create_connection()),
                Err(_) => insert_into(secrets::dsl::secrets)
                    .values(&secret)
                    .execute(&mut create_connection()),
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

fn remove() -> Command {
    Command::new("remove")
        .description("Remove a repository secret")
        .action(|context| {
            let auth = match authorize() {
                Ok(auth) => auth,
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

            let project_relative_path: String = PathBuf::from(&auth.folder.unwrap_or_default())
                .join(&cwd_relative_path)
                .to_str()
                .unwrap()
                .into();
            match delete(secrets::dsl::secrets)
                .filter(secrets::project.eq(&auth.project))
                .filter(secrets::path.eq(&project_relative_path))
                .execute(&mut create_connection())
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

pub fn secrets() -> Command {
    Command::new("secrets")
        .description("Manage secrets for different repositories")
        .command(list())
        .command(clone())
        .command(set())
        .command(remove())
        .action(|context| context.help())
}
