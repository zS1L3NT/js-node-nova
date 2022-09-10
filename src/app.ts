import { Command } from "cliffy/command/mod.ts"

import config from "./commands/config.ts"
import generate from "./commands/generate.ts"
import secret from "./commands/secret.ts"

const nova = new Command()
	.name("nova")
	.description("A CLI for helping me with various tasks")
	.action(() => {
		nova.showHelp()
	})
	.command("config", config)
	.command("generate", generate)
	.command("secret", secret)

await nova.parse(Deno.args)
