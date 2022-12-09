use {
    parson::{JSONObject, JSONValue},
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
    let cwd = current_dir().unwrap();
    let text = fs::read_to_string(cwd.join("package.json")).ok()?;
    let package = text.parse::<JSONValue>().ok()?;
    let package = package.get_object().ok()?;

    let dependencies = package.get("dependencies")?;
    let dependencies = dependencies.get_object().ok()?.to_hashmap();
    let dependencies = dependencies
        .iter()
        .map(|(k, v)| (k.to_string(), v.get_string().unwrap()))
        .collect::<HashMap<_, _>>();

    let default_dev_dependencies = JSONValue::from_object(JSONObject::new());
    let dev_dependencies = package
        .get("devDependencies")
        .unwrap_or(&default_dev_dependencies);
    let dev_dependencies = dev_dependencies.get_object().ok()?.to_hashmap();
    let dev_dependencies = dev_dependencies
        .iter()
        .map(|(k, v)| (k.to_string(), v.get_string().unwrap()))
        .collect::<HashMap<_, _>>();

    let dependencies = dependencies
        .into_iter()
        .chain(dev_dependencies.into_iter())
        .collect::<HashMap<_, _>>();

    let mut dependency_names = dependencies.keys().collect::<Vec<_>>();
    dependency_names.sort();

    for dependency in dependency_names {
		let version = dependencies.get(dependency).unwrap();
        println!("\t-   [![{}](https://img.shields.io/badge/{}-{}-red?style=flat-square)](https://npmjs.com/package/{}/v/{})",
			dependency,
			using_clean_url(dependency),
			using_clean_url(version),
			dependency,
			version
		);
    }

    Some(())
}

fn read_cargo_toml() -> Option<()> {
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

        println!("\t-   [![{}](https://img.shields.io/badge/{}-{}-blue?style=flat-square)](https://crates.io/crates/{}/{})",
			dependency,
			using_clean_url(dependency),
			using_clean_url(version),
			dependency,
			version,
		);
    }

    Some(())
}

pub fn generate() -> Command {
    Command::new("generate")
        .description("Generate the `Built with` section for my README.md files")
        .action(|_| {
            if read_package_json().is_some() {
                return;
            }

            if read_cargo_toml().is_some() {
                return;
            }

            println!("No package.json or cargo.toml found");
        })
}
