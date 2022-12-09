use {
    seahorse::Command,
    std::{collections::HashMap, env::current_dir, fs},
};

fn using_clean_url<T>(text: T) -> String
where
    T: Into<String>,
{
    urlencoding::encode(&text.into())
        .replace('-', "--")
        .replace('_', "__")
}

fn read_package_json() -> Option<()> {
    let mut output = String::new();

    let cwd = current_dir().unwrap();
    let text = fs::read_to_string(cwd.join("package.json")).ok()?;
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

    clipboard_win::set_clipboard_string(&output[..output.len() - 1]).unwrap();
    println!("Copied package.json data to clipboard");
    Some(())
}

fn read_pubspec_yaml() -> Option<()> {
    let mut output = String::new();

    let cwd = current_dir().unwrap();
    let text = fs::read_to_string(cwd.join("pubspec.yaml")).ok()?;
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
        .filter(|(k, _)| k.as_str().unwrap() != "flutter" && k.as_str().unwrap() != "flutter_test")
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

    clipboard_win::set_clipboard_string(&output[..output.len() - 1]).unwrap();
    println!("Copied pubspec.yaml data to clipboard");
    Some(())
}

fn read_cargo_toml() -> Option<()> {
    let mut output = String::new();

    let cwd = current_dir().unwrap();
    let text = fs::read_to_string(cwd.join("Cargo.toml")).ok()?;
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

    clipboard_win::set_clipboard_string(&output[..output.len() - 1]).unwrap();
    println!("Copied Cargo.toml data to clipboard");
    Some(())
}

pub fn generate() -> Command {
    Command::new("generate")
        .description("Generate the `Built with` section for my README.md files")
        .action(|_| {
            if read_package_json().is_some() {
                return;
            }

            if read_pubspec_yaml().is_some() {
                return;
            }

            if read_cargo_toml().is_some() {
                return;
            }

            println!("No package.json, pubspec.yaml or cargo.toml found");
        })
}
