use {
    crate::{error, success},
    clipboard::{ClipboardContext, ClipboardProvider},
};

fn using_clean_url<T>(text: T) -> String
where
    T: Into<String>,
{
    urlencoding::encode(&text.into())
        .replace('-', "--")
        .replace('_', "__")
}

fn read_package_json(text: String) -> Option<()> {
    let mut output = String::new();

    let json = serde_json::from_str::<serde_json::Value>(&text).ok()?;

    let empty_object = serde_json::Value::Object(serde_json::map::Map::new());
    let dependencies = json
        .as_object()?
        .get("dependencies")?
        .as_object()?
        .iter()
        .chain(
            json.as_object()?
                .get("devDependencies")
                .unwrap_or(&empty_object)
                .as_object()?
                .iter(),
        )
        .map(|(k, v)| (k.as_str(), v.as_str().unwrap()))
        .collect::<std::collections::HashMap<_, _>>();

    let mut dependency_names = dependencies.keys().collect::<Vec<_>>();
    dependency_names.sort();

    for dependency in dependency_names {
        let dependency = *dependency;
        let version = *dependencies.get(dependency).unwrap();

        output.push_str(&format!("        -   [![{}](https://img.shields.io/badge/{}-{}-red?style=flat-square)](https://npmjs.com/package/{}/v/{})\n",
			dependency,
			using_clean_url(dependency),
			using_clean_url(version),
			dependency,
			version.replace(['^', '~'], "")
		));
    }

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(output[..output.len() - 1].into()).unwrap();
    ctx.get_contents().unwrap();
    success!("Copied package.json data to clipboard");
    Some(())
}

fn read_pubspec_yaml(text: String) -> Option<()> {
    let mut output = String::new();

    let yaml = serde_yaml::from_str::<serde_yaml::Value>(&text).ok()?;

    let dependencies = yaml
        .as_mapping()?
        .get("dependencies")?
        .as_mapping()?
        .iter()
        .chain(
            yaml.as_mapping()?
                .get("dev_dependencies")?
                .as_mapping()?
                .iter(),
        )
        .filter(|(_, v)| v.is_string())
        .map(|(k, v)| (k.as_str().unwrap(), v.as_str().unwrap()))
        .collect::<std::collections::HashMap<_, _>>();

    let mut dependency_names = dependencies.keys().collect::<Vec<_>>();
    dependency_names.sort();

    for dependency in dependency_names {
        let dependency = *dependency;
        let version = *dependencies.get(dependency).unwrap();

        output.push_str(&format!("        -   [![{}](https://img.shields.io/badge/{}-{}-blue?style=flat-square)](https://pub.dev/packages/{}/versions/{})\n",
			dependency,
			using_clean_url(dependency),
			using_clean_url(version),
			dependency,
			version.replace(['^', '~'], ""),
		));
    }

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(output[..output.len() - 1].into()).unwrap();
    ctx.get_contents().unwrap();
    Some(())
}

fn read_cargo_toml(text: String) -> Option<()> {
    let mut output = String::new();

    let cargo = text.parse::<toml::Value>().ok()?;

    let dependencies = cargo.get("dependencies")?.as_table()?;
    for (dependency, version) in dependencies {
        let version = if version.is_str() {
            version.as_str()
        } else {
            version.as_table()?.get("version")?.as_str()
        }?;

        output.push_str(&format!("        -   [![{}](https://img.shields.io/badge/{}-{}-yellow?style=flat-square)](https://crates.io/crates/{}/{})\n",
			dependency,
			using_clean_url(dependency),
			using_clean_url(version),
			dependency,
			version.replace(['^', '~'], ""),
		));
    }

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(output[..output.len() - 1].into()).unwrap();
    ctx.get_contents().unwrap();
    success!("Copied Cargo.toml data to clipboard");
    Some(())
}

fn read_build_gradle(text: String) -> Option<()> {
    let mut output = String::new();

    let mut reading_dependencies = false;
    for line in text.split('\n') {
        let line = line.trim();

        if reading_dependencies {
            if line.trim() == "}" {
                break;
            }

            let regex = regex::Regex::new(
                r#"^\w+ (?:['"](.+):(.+):(.+)['"]|\w+\(['"](.+):(.+):(.+)['"]\))$"#,
            )
            .unwrap();
            if regex.is_match(line) {
                let captures = regex.captures(line).unwrap();
                let group = captures
                    .get(1)
                    .unwrap_or_else(|| captures.get(4).unwrap())
                    .as_str();
                let dependency = captures
                    .get(2)
                    .unwrap_or_else(|| captures.get(5).unwrap())
                    .as_str();
                let version = captures
                    .get(3)
                    .unwrap_or_else(|| captures.get(6).unwrap())
                    .as_str();

                output.push_str(&format!("        -   [![{}:{}](https://img.shields.io/badge/{}-{}-brightgreen?style=flat-square)](https://mvnrepository.com/artifact/{}/{}/{})\n",
				group,
				dependency,
				using_clean_url(format!("{}:{}", group, dependency)),
				using_clean_url(version),
				group,
				dependency,
				version,
			));
            } else if line.is_empty() || line.starts_with("//") {
                continue;
            } else {
                error!("Failed to parse dependency", line);
            }
        } else if line == "dependencies {" {
            reading_dependencies = true;
            continue;
        }
    }

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(output[..output.len() - 1].into()).unwrap();
    ctx.get_contents().unwrap();
    success!("Copied build.gradle data to clipboard");
    Some(())
}

pub fn generate() -> seahorse::Command {
    seahorse::Command::new("generate")
        .description("Generate the `Built with` section for my README.md files")
        .usage("nova generate [path/to/file]")
        .action(|context| {
            let path = match context.args.first() {
                Some(path) => std::path::PathBuf::from(path),
                None => {
                    error!("Please provide a filename");
                    return;
                }
            };

            let file = match std::fs::read_to_string(&path) {
                Ok(file) => file,
                Err(err) => {
                    error!("Unable to read from file", path.display(); err);
                    return;
                }
            };

            if path.file_name().is_none() {
                error!("Cannot parse folder", path.display());
                return;
            }

            match path.file_name().unwrap().to_str().unwrap() {
                "package.json" => {
                    if read_package_json(file).is_none() {
                        error!("Unable to parse package.json", path.display());
                    }
                }
                "pubspec.yaml" => {
                    if read_pubspec_yaml(file).is_none() {
                        error!("Unable to parse pubspec.yaml", path.display());
                    }
                }
                "Cargo.toml" => {
                    if read_cargo_toml(file).is_none() {
                        error!("Unable to parse Cargo.toml", path.display());
                    }
                }
                "build.gradle" => {
                    if read_build_gradle(file).is_none() {
                        error!("Unable to parse build.gradle", path.display());
                    }
                }
                _ => {
                    error!("Unable to parse file", path.display());
                }
            };
        })
}
