import { Command } from "https://deno.land/x/cliffy@v0.25.0/command/mod.ts"

const clone = new Command()
	.name("clone")
	.description("Clone project configuration file(s) to the current working directory")
	.arguments("<shorthand...>")
	.action((_, ...shorthands) => {
		console.log("Shorthands:", shorthands)
	})

const list = new Command()
	.name("list")
	.description("List all project configuration file(s) and their shorthands")
	.action(_ => {
		console.log("List")
	})

const config = new Command()
	.name("config")
	.description("Manage reusable project configuration files")
	.action(_ => {
		config.showHelp()
	})
	.command("clone", clone)
	.command("list", list)

export default config
