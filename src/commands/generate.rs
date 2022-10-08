use {
    parson::{JSONObject, JSONValue},
    seahorse::Command,
    std::{collections::HashMap, env, fs},
};

fn read_package_json() -> Option<()> {
    let cwd = env::current_dir().unwrap();
    let text = fs::read_to_string(cwd.join("package.json")).ok()?;
    let package = text.parse::<JSONValue>().ok()?;
    let package = package.get_object().ok()?;

    let dependencies = package.get("dependencies")?;
    let dependencies = dependencies.get_object().ok()?.to_hashmap();
    let dependencies = dependencies
        .iter()
        .map(|(k, _)| (k.to_string(), true))
        .collect::<HashMap<String, bool>>();

    let default_dev_dependencies = JSONValue::from_object(JSONObject::new());
    let dev_dependencies = package
        .get("devDependencies")
        .unwrap_or(&default_dev_dependencies);
    let dev_dependencies = dev_dependencies.get_object().ok()?.to_hashmap();
    let dev_dependencies = dev_dependencies
        .iter()
        .map(|(k, _)| (k.to_string(), false))
        .collect::<HashMap<String, bool>>();

    let all_dependencies = dependencies
        .into_iter()
        .chain(dev_dependencies.into_iter())
        .collect::<HashMap<_, _>>();

    let mut all_dependency_names = all_dependencies.keys().collect::<Vec<_>>();
    all_dependency_names.sort();

    for dependency_name in all_dependency_names {
        println!("\t-   [![{}](https://img.shields.io/github/package-json/dependency-version/zS1L3NT/{}{}{}?style=flat-square)](https://npmjs.com/package/{})",
			dependency_name,
			cwd.to_str()?.split('\\').last().unwrap(),
			if *all_dependencies.get(dependency_name).unwrap() { "/" } else { "/dev/" },
			dependency_name,
			dependency_name
		);
    }

    Some(())
}

fn read_cargo_toml() -> Option<()> {
    let cwd = env::current_dir().unwrap();
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
			urlencoding::encode(dependency).replace('-', "--"),
			urlencoding::encode(version).replace('-', "--"),
			urlencoding::encode(dependency).replace('-', "--"),
			urlencoding::encode(version).replace('-', "--"),
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
