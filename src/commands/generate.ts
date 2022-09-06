import { Command } from "https://deno.land/x/cliffy@v0.25.0/command/mod.ts"

export default new Command()
	.name("generate")
	.description("Generate the `Built with` section for my README.md files")
	.arguments("[directory]")
	.action((_, directory) => {
		console.log("Generate:", directory)
	})
