use regex::Regex;

use {
	clipboard::{ClipboardProvider, ClipboardContext},
    seahorse::Command,
    std::{
        collections::HashMap,
        fs,
        path::{Path, PathBuf},
    },
};

fn using_clean_url<T>(text: T) -> String
where
    T: Into<String>,
{
    urlencoding::encode(&text.into())
        .replace('-', "--")
        .replace('_', "__")
}

fn read_package_json(folder: &Path) -> Option<()> {
    let mut output = String::new();

    let text = fs::read_to_string(folder.join("package.json")).ok()?;
    let json = serde_json::from_str::<serde_json::Value>(&text).ok()?;

    let dependencies = json
        .as_object()?
        .get("dependencies")?
        .as_object()?
        .iter()
        .chain(
            json.as_object()?
                .get("devDependencies")?
                .as_object()?
                .iter(),
        )
        .map(|(k, v)| (k.as_str(), v.as_str().unwrap()))
        .collect::<HashMap<_, _>>();

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
			if let Some(version) = version.strip_prefix('^') {version} else {version}
		));
    }

	let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(output[..output.len() - 1].into()).unwrap();
	ctx.get_contents().unwrap();
    println!("Copied package.json data to clipboard");
    Some(())
}

fn read_pubspec_yaml(folder: &Path) -> Option<()> {
    let mut output = String::new();

    let text = fs::read_to_string(folder.join("pubspec.yaml")).ok()?;
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
        .collect::<HashMap<_, _>>();

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
			if let Some(version) = version.strip_prefix('^') {version} else {version},
		));
    }

	let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(output[..output.len() - 1].into()).unwrap();
	ctx.get_contents().unwrap();
    Some(())
}

fn read_cargo_toml(folder: &Path) -> Option<()> {
    let mut output = String::new();

    let text = fs::read_to_string(folder.join("Cargo.toml")).ok()?;
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
			version,
		));
    }

	let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(output[..output.len() - 1].into()).unwrap();
	ctx.get_contents().unwrap();
    println!("Copied Cargo.toml data to clipboard");
    Some(())
}

fn read_build_gradle(folder: &Path) -> Option<()> {
	let mut output = String::new();

    let text = fs::read_to_string(folder.join("build.gradle")).ok()?;

	let mut reading_dependencies = false;
	for line in text.split('\n') {
		let line = line.trim();

		if reading_dependencies {
			if line.trim() == "}" {
				break;
			}

			let regex = Regex::new(r#"^\w+ (?:['"](.+):(.+):(.+)['"]|\w+\(['"](.+):(.+):(.+)['"]\))$"#).unwrap();
			if regex.is_match(line) {
				let captures = regex.captures(line).unwrap();
				let group = captures.get(1).unwrap_or_else(|| captures.get(4).unwrap()).as_str();
				let dependency = captures.get(2).unwrap_or_else(|| captures.get(5).unwrap()).as_str();
				let version = captures.get(3).unwrap_or_else(|| captures.get(6).unwrap()).as_str();

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
				println!("Failed to parse dependency: {}", line);
			}
		} else if line == "dependencies {" {
			reading_dependencies = true;
			continue;
		}
	}

	let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(output[..output.len() - 1].into()).unwrap();
	ctx.get_contents().unwrap();
    println!("Copied build.gradle data to clipboard");
    Some(())
}

pub fn generate() -> Command {
    Command::new("generate")
        .description("Generate the `Built with` section for my README.md files")
        .action(|context| {
            match context.args.first() {
                Some(path) => {
                    let path = PathBuf::from(path);

                    if read_package_json(&path).is_some() {
                        return;
                    }

                    if read_pubspec_yaml(&path).is_some() {
                        return;
                    }

                    if read_cargo_toml(&path).is_some() {
                        return;
                    }

					if read_build_gradle(&path).is_some() {
						return;
					}

                    println!("No package.json, pubspec.yaml, cargo.toml or build.gradle found");
                }
                None => {
                    println!("No path provided");
                }
            };
        })
}
