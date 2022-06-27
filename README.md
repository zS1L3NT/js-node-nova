# Nova CLI

![License](https://img.shields.io/github/license/zS1L3NT/ts-deno-nova?style=for-the-badge) ![Languages](https://img.shields.io/github/languages/count/zS1L3NT/ts-deno-nova?style=for-the-badge) ![Top Language](https://img.shields.io/github/languages/top/zS1L3NT/ts-deno-nova?style=for-the-badge) ![Commit Activity](https://img.shields.io/github/commit-activity/y/zS1L3NT/ts-deno-nova?style=for-the-badge) ![Last commit](https://img.shields.io/github/last-commit/zS1L3NT/ts-deno-nova?style=for-the-badge)

Nova is my personal CLI for importing project config files into my new projects. I have a batch file on my Windows Computer to run Nova CLI with the `nova` command.

## Motivation

I have had many TypeScript projects, and copying the config files into new projects can be tiring. Plus, I wanted to have somewhere I can store the most up-to-date version of these project config files. Using Nova CLI, I can always import the latest version of these config files into fresh or outdated projects.

## Features

-   Writing to config files
    -   `nova add [one or more file commands]`
        -   `ts` - Adds my tsconfig.json file
        -   `git` - Adds my .gitignore file
        -   `pkg` - Adds my generic package.json file
        -   `env` - Adds my .editorconfig and .prettierrc files
    -   `nova copy [one or more file commands]`
        -   `debugger` - Prints the debugger.json to the console

## Usage

To use Nova CLI, run this command

```
$ deno run --allow-read --allow-write src/app.ts [args]
```

## Built with

-   Deno
