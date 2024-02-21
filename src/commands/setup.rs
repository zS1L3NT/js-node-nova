use crate::{error, success, warn};

static AUTHOR: &'static str = "zS1L3NT <dev@zectan.com> (https://www.zectan.com)";
static LICENSE: &'static str = "GPL-3.0";
static SCRIPT_LINT: &'static str =
    "tsc --noEmit && rm tsconfig.tsbuildinfo && eslint src --fix && prettier src --write";
static DEV_DEPENDENCIES: [&'static str; 10] = [
    "@typescript-eslint/eslint-plugin",
    "@typescript-eslint/parser",
    "bun-types",
    "eslint",
    "eslint-config-next",
    "eslint-config-prettier",
    "eslint-plugin-react",
    "eslint-plugin-simple-import-sort",
    "prettier",
    "typescript",
];

pub fn setup() -> seahorse::Command {
    seahorse::Command::new("setup")
        .description("Setup NPM package.json for my own custom project")
        .usage("nova setup [npm-cli] [path/to/package.json]")
        .action(|context| {
            let cli = match context.args.first() {
                Some(string) => match string.as_ref() {
                    "bun" => "bun",
                    "pnpm" => "pnpm",
                    "yarn" => "yarn",
                    "npm" => "npm",
                    _ => {
                        error!("Unknown npm cli provided", string);
                        return;
                    }
                },
                None => {
                    error!("Please provide an npm cli");
                    return;
                }
            };

            let path = match context.args.get(1) {
                Some(path) => match std::path::PathBuf::from(path).canonicalize() {
                    Ok(path) => path,
                    Err(err) => {
                        error!("Unable to parse package.json path"; err);
                        return;
                    }
                },
                None => {
                    error!("Please provide a package.json path");
                    return;
                }
            };

            let old = match std::fs::read_to_string(&path) {
                Ok(file) => match json::parse(&file) {
                    Ok(json) => json,
                    Err(err) => {
                        error!("Unable to parse package.json"; err);
                        return;
                    }
                },
                Err(err) => {
                    error!("Unable to read from package.json"; err);
                    return;
                }
            };

            let mut description = String::new();
            let mut reactjs = false;
            let mut nextjs = false;
            print!("Description: ");
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            std::io::stdin().read_line(&mut description).unwrap();

            let mut new = json::object! {
                name: path.parent().unwrap().file_name().unwrap().to_str().unwrap(),
                description: description.trim(),
                author: AUTHOR,
                license: LICENSE,
            };

            let mut new_scripts = json::object! {};
            if old.has_key("scripts") {
                let old_scripts = &old["scripts"];
                if old_scripts.is_object() {
                    let mut has_lint = false;
                    for (key, value) in old_scripts.entries() {
                        if key == "lint" {
                            has_lint = true;
                            new_scripts.insert(key, SCRIPT_LINT).unwrap();
                        } else {
                            new_scripts.insert(key, value.clone()).unwrap();
                        }
                    }

                    if !has_lint {
                        new_scripts.insert("lint", SCRIPT_LINT).unwrap();
                    }
                } else {
                    warn!("\"scripts\" property is not an object");
                    new_scripts.insert("lint", SCRIPT_LINT).unwrap();
                }
            } else {
                new_scripts.insert("lint", SCRIPT_LINT).unwrap();
            }
            new.insert("scripts", new_scripts).unwrap();

            for dep_key in ["dependencies", "devDependencies"] {
                let mut new_deps: json::JsonValue = json::object! {};
                if old.has_key(dep_key) {
                    let old_deps = &old[dep_key];
                    if old_deps.is_object() {
                        for (key, value) in old_deps.entries() {
                            if key == "react" {
                                reactjs = true;
                            }
                            if key == "next" {
                                nextjs = true;
                            }
                            if DEV_DEPENDENCIES.contains(&key) {
                                continue;
                            }
                            new_deps.insert(key, value.clone()).unwrap();
                        }
                    } else {
                        warn!(format!("\"{}\" property is not an object...", dep_key));
                    }
                }
                new.insert(dep_key, new_deps).unwrap();
            }

            if let Err(err) = std::fs::write(&path, format!("{}", new)) {
                error!("Unable to write to package.json"; err);
                return;
            }

            let command = format!(
                "cd {} && {} i -D {}",
                path.parent().unwrap().to_str().unwrap(),
                cli,
                DEV_DEPENDENCIES
                    .iter()
                    .filter(|d| if *d == &"eslint-config-next" {
                        nextjs
                    } else if *d == &"eslint-plugin-react" {
                        reactjs
                    } else {
                        true
                    })
                    .map(|d| *d)
                    .collect::<Vec<&str>>()
                    .join(" "),
            );

            let mut child = match std::process::Command::new("/bin/bash")
                .arg("-c")
                .arg(command)
                .spawn()
            {
                Ok(child) => child,
                Err(err) => {
                    error!("Unable to run install command"; err);
                    return;
                }
            };

            match child.wait() {
                Ok(_) => {}
                Err(err) => {
                    error!("Install command exited with error"; err);
                    return;
                }
            }

            if let Err(err) = std::fs::write(
                &path,
                json::parse(&std::fs::read_to_string(&path).unwrap())
                    .unwrap()
                    .pretty(4)
                    .replace("    ", "\t"),
            ) {
                error!("Unable to write to package.json"; err);
                return;
            }

            if let Err(err) = std::os::unix::fs::chown(&path, Some(501), Some(20)) {
                error!("Unable to change file owner", path.to_str().unwrap(); err);
                return;
            }

            success!("Modified package.json");
        })
}
