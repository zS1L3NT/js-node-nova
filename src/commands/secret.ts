import { Command } from "https://deno.land/x/cliffy@v0.25.0/command/mod.ts"

const clone = new Command()
	.name("clone")
	.description("Clone the repository secret to the original location")
	.action(_ => {
		console.log("Clone")
	})

const set = new Command()
	.name("set")
	.description("Set the repository secret, update if it already exists")
	.arguments("<path>")
	.action((_, path) => {
		console.log("Set:", path)
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
