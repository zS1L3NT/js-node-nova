# Nova CLI

![License](https://img.shields.io/github/license/zS1L3NT/ts-deno-nova?style=for-the-badge) ![Languages](https://img.shields.io/github/languages/count/zS1L3NT/ts-deno-nova?style=for-the-badge) ![Top Language](https://img.shields.io/github/languages/top/zS1L3NT/ts-deno-nova?style=for-the-badge) ![Commit Activity](https://img.shields.io/github/commit-activity/y/zS1L3NT/ts-deno-nova?style=for-the-badge) ![Last commit](https://img.shields.io/github/last-commit/zS1L3NT/ts-deno-nova?style=for-the-badge)

Nova is my personal CLI for importing project config files into my new projects. I have a batch file on my Windows Computer to run Nova CLI with the `nova` command. Nova also helps me import project secrets back when I clone them from GitHub again or onto another machine.

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
	-   `nova generate [location]`
		-	NodeJS Projects
		-	DenoJS Projects
		-	Dart Projects
-	Cloning a project secret file
	-	`nova secret clone`
-	Setting a project secret file
	-	`nova secret set [file path]`

## Usage

To use Nova CLI, run this command

```
$ nova.bat [args]
```

## Built with

-   Deno
	-   CLI
        -   [![cliffy](https://img.shields.io/badge/cliffy-0.25.0-blue?style=flat-square)](https://deno.land/x/cliffy@v0.25.0)
	-   PostGres
        -   [![denodb](https://img.shields.io/badge/denodb-1.0.40-blue?style=flat-square)](https://deno.land/x/denodb@v1.0.40)
        -   [![dotenv](https://img.shields.io/badge/dotenv-3.2.0-blue?style=flat-square)](https://deno.land/x/dotenv@v3.2.0)
	-   Encryption
        -   [![god_crypto](https://img.shields.io/badge/god_crypto-1.4.10-blue?style=flat-square)](https://deno.land/x/god_crypto@v1.4.10)