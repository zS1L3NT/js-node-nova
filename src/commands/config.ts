import { Command } from "https://deno.land/x/cliffy@v0.25.0/command/mod.ts"
import { Row } from "https://deno.land/x/cliffy@v0.25.0/table/row.ts"
import { Table } from "https://deno.land/x/cliffy@v0.25.0/table/table.ts"

import { Config } from "../postgres.ts"

const clone = new Command()
	.name("clone")
	.description("Clone project configuration file(s) to the current working directory")
	.arguments("<shorthand...>")
	.action(async (_, ...shorthands) => {
		for (const shorthand of shorthands) {
			try {
				const config = await Config.where({ shorthand }).first()
				if (config) {
					Deno.writeTextFileSync(<string>config.filename, <string>config.content)
					console.log(`Wrote to file: ${config.filename}`)
				} else {
					console.log(`Unknown config shorthand: ${shorthand}`)
				}
			} catch (err) {
				console.log(err)
			}
		}
	})

const list = new Command()
	.name("list")
	.description("List all project configuration file(s) and their shorthands")
	.action(async _ => {
		const configs = await Config.select("shorthand", "filename").all()
		new Table()
			.header(Row.from(["Shorthand", "Filename"]).border(true))
			.body(configs.map(config => [<string>config.shorthand, <string>config.filename]))
			.render()
		console.log()
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
