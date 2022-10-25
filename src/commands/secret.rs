use {
    super::super::{
        aes::{decrypt, encrypt, validate},
        create_connection,
        models::Secret,
        schema::secrets,
    },
    dialoguer::{theme::ColorfulTheme, Password},
    diesel::{insert_into, prelude::*},
    regex::Regex,
    seahorse::Command,
    std::{env::current_dir, fs, path::PathBuf},
};

fn clone() -> Command {
    Command::new("clone")
        .description("Clone the repository secret to the original location")
        .action(|_| {
            let key = Password::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter password: ")
                .interact()
                .unwrap();

            if !validate(&key) {
                println!("Incorrect key");
                return;
            }

            let regex = Regex::new(r#"^C:\\Projects\\([^\\]*)$"#).unwrap();
            if !regex.is_match(current_dir().unwrap().to_str().unwrap()) {
                println!("Invalid project path");
                return;
            }

            let directory = current_dir().unwrap();
            let project = match regex.captures(directory.to_str().unwrap()) {
                Some(captures) => captures.get(1).unwrap().as_str(),
                None => {
                    println!("Invalid project path");
                    return;
                }
            };

            let secret = match secrets::dsl::secrets
                .filter(secrets::project.eq(project))
                .first::<Secret>(&mut create_connection())
            {
                Ok(secret) => secret,
                Err(_) => {
                    println!("No secret found for this project");
                    return;
                }
            };

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
        })
}

fn set() -> Command {
    Command::new("set")
        .description("Set the repository secret, update if it already exists")
        .action(|context| {
            let regex = Regex::new(r#"^C:\\Projects\\([^\\]*)$"#).unwrap();
            if !regex.is_match(current_dir().unwrap().to_str().unwrap()) {
                println!("Invalid project path");
                return;
            }

            let directory = current_dir().unwrap();
            let project = match regex.captures(directory.to_str().unwrap()) {
                Some(captures) => captures.get(1).unwrap().as_str().into(),
                None => {
                    println!("Invalid project path");
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

            let key = Password::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter password: ")
                .interact()
                .unwrap();

            if !validate(&key) {
                println!("Incorrect key");
                return;
            }

            let secret = Secret {
                project,
                path: path.clone(),
                content: encrypt(&content, &key).unwrap(),
            };

            match insert_into(secrets::dsl::secrets)
                .values(&secret)
                .on_conflict(secrets::project)
                .do_update()
                .set(&secret)
                .execute(&mut create_connection())
            {
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

pub fn secret() -> Command {
    Command::new("secret")
        .description("Manage secrets for different repositories")
        .command(clone())
        .command(set())
        .action(|context| context.help())
}
