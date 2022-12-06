# Nova CLI

![License](https://img.shields.io/github/license/zS1L3NT/rs-cli-nova?style=for-the-badge) ![Languages](https://img.shields.io/github/languages/count/zS1L3NT/rs-cli-nova?style=for-the-badge) ![Top Language](https://img.shields.io/github/languages/top/zS1L3NT/rs-cli-nova?style=for-the-badge) ![Commit Activity](https://img.shields.io/github/commit-activity/y/zS1L3NT/rs-cli-nova?style=for-the-badge) ![Last commit](https://img.shields.io/github/last-commit/zS1L3NT/rs-cli-nova?style=for-the-badge)

Nova is my personal CLI for importing project config files into my new projects. Nova also helps me import project secrets back when I clone them from GitHub again or onto another machine.

## Motivation

I have had many TypeScript projects, and copying the config files into new projects can be tiring. Plus, I wanted to have somewhere I can store the most up-to-date version of these project config files. Using Nova CLI, I can always import the latest version of these config files into fresh or outdated projects.

## Features

-   Writing to config files
    -   `nova config clone [one or more file commands]`
        -   `ts` - Adds my tsconfig.json file
        -   `git` - Adds my .gitignore file
        -   `pkg` - Adds my generic package.json file
        -   `ecf` - Adds my .editorconfig file
		-   and many more...
-   Listing all config files
    -   `nova config list`
-   Generating a list of dependencies for my README.md files
	-   `nova generate`
		-	NodeJS Projects
		-	DenoJS Projects
		-	Dart Projects
-	Listing all project secret files
	-	`nova secrets list`
-	Cloning a project secret file
	-	`nova secrets clone`
-	Setting a project secret file
	-	`nova secrets set [file path]`
-	Removing a project secret file
	-	`nova secrets set [file path]`

## Usage

To use Nova CLI, run this command

```
$ cargo run
```

Do note that the script needs the environment variables from [.env.example](.env.example)

## Built with

-   Rust
    - Encryption
        -   [![aes-gcm](https://img.shields.io/badge/aes--gcm-0.10.1-blue?style=flat-square)](https://crates.io/crates/aes--gcm/0.10.1)
        -   [![base64](https://img.shields.io/badge/base64-0.13.0-blue?style=flat-square)](https://crates.io/crates/base64/0.13.0)
	- Database
        -   [![diesel](https://img.shields.io/badge/diesel-2.0.0-blue?style=flat-square)](https://crates.io/crates/diesel/2.0.0)
	- Text Parsing
        -   [![parson](https://img.shields.io/badge/parson-1.1.0-blue?style=flat-square)](https://crates.io/crates/parson/1.1.0)
        -   [![toml](https://img.shields.io/badge/toml-0.5.9-blue?style=flat-square)](https://crates.io/crates/toml/0.5.9)
        -   [![urlencoding](https://img.shields.io/badge/urlencoding-2.1.2-blue?style=flat-square)](https://crates.io/crates/urlencoding/2.1.2)
	- Miscellaneous
        -   [![dialoguer](https://img.shields.io/badge/dialoguer-0.10.2-blue?style=flat-square)](https://crates.io/crates/dialoguer/0.10.2)
        -   [![regex](https://img.shields.io/badge/regex-1.6.0-blue?style=flat-square)](https://crates.io/crates/regex/1.6.0)
        -   [![prettytable-rs](https://img.shields.io/badge/prettytable--rs-%5E0.9.0-blue?style=flat-square)](https://crates.io/crates/prettytable-rs/0.9.0)
        -   [![seahorse](https://img.shields.io/badge/seahorse-2.1-blue?style=flat-square)](https://crates.io/crates/seahorse/2.1)