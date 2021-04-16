const args = process.argv.slice(2)
const fs = require("fs")
const path = require("path")

const copy = filename => {
	fs.copyFile(
		path.join(__dirname, "add", filename),
		filename,
		() => console.log(`Wrote to ${filename}`)
	)
}

if (args.length === 0) return console.log("No arguments passed")
if (args[0] === "add") {
	if (args.length === 1) {
		console.log("No shortcut arguements passed")
	}
	for (let i = 1, il = args.length; i < il; i++) {
		const arg = args[i]
		if (arg === "ts") {
			copy("tsconfig.json")
			continue
		}
		if (arg === "git") {
			copy(".gitignore")
			continue
		}
		if (arg === "env") {
			copy(".editorconfig")
			copy(".prettierrc")
			copy("nodemon.json")
			continue
		}
		if (arg === "pkg") {
			copy("package.json")
			continue
		}
		console.log(`No such shortcut: (${arg})`)
	}
} else if (args[0] === "copy") {
	if (args.length === 1) {
		console.log("No shortcut arguements passed")
	}
	for (let i = 1, il = args.length; i < il; i++) {
		const arg = args[i]
		if (arg === "debug") {
			const data = fs.readFileSync(path.join(__dirname, "copy", "debug.json"), 'utf-8')
			console.log(data)
		}
	}
} else {
	console.log(`No such command (${args[0]})`)
}
