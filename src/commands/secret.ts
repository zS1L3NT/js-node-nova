import { Command } from "https://deno.land/x/cliffy@v0.25.0/command/mod.ts"
import { Secret as SecretPrompt } from "https://deno.land/x/cliffy@v0.25.0/prompt/mod.ts"

import { decrypt, encrypt } from "../aes.ts"
import { Secret } from "../postgres.ts"

const clone = new Command()
	.name("clone")
	.description("Clone the repository secret to the original location")
	.action(async _ => {
		const key = await SecretPrompt.prompt("Enter encryption key: ")
		if (key !== Deno.env.get("AES__KEY")) {
			console.log("Incorrect key")
			return
		}

		const match = Deno.cwd().match(/^C:\\Projects\\([^\\]*)$/)
		if (!match) {
			console.log("Invalid project path")
			return
		}

		const secret = await Secret.where({ project: match[1]! }).first()
		if (!secret) {
			console.log("No secret found for this project")
			return
		}

		Deno.writeTextFileSync(<string>secret.path, await decrypt(<string>secret.content, key))
	})

const set = new Command()
	.name("set")
	.description("Set the repository secret, update if it already exists")
	.arguments("<path>")
	.action(async (_, path) => {
		const key = await SecretPrompt.prompt("Enter encryption key: ")
		if (key !== Deno.env.get("AES__KEY")) {
			console.log("Incorrect key")
			return
		}

		const match = Deno.cwd().match(/^C:\\Projects\\([^\\]*)$/)
		if (!match) {
			console.log("Invalid project path")
			return
		}

		try {
			await Secret.create({
				project: match[1]!,
				path,
				content: await encrypt(Deno.readTextFileSync(path), key)
			})
			console.log("Stored encrypted secret file:", path)
		} catch (err) {
			console.error(err)
		}
	})

const secret = new Command()
	.name("secret")
	.description("Manage secrets for different repositories")
	.action(_ => {
		secret.showHelp()
	})
	.command("clone", clone)
	.command("set", set)

export default secret
