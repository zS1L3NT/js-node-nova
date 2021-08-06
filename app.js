const args = process.argv.slice(2)
const fs = require("fs")
const path = require("path")

const add = filename => {
	fs.copyFile(
		path.join(__dirname, "add", filename + ".bkp"),
		filename,
		() => console.log(`Wrote to ${filename}`)
	)
}

const copy = filename => {
	console.log(
		fs.readFileSync(
			path.join(__dirname, "copy", filename),
			'utf-8'
		)
	)
}

if (args.length === 0) return console.log("No arguments passed")
if (args[0] === "add") {
	if (args.length === 1) {
		console.log("No shortcut arguements passed")
	}
	for (let i = 1, il = args.length; i < il; i++) {
		const arg = args[i]
		switch (arg) {
			case "ts":
				add("tsconfig.json")
				break
			case "git":
				add(".gitignore")
				break
			case "env":
				add(".editorconfig")
				add(".prettierrc")
				add("nodemon.json")
				break
			case "pkg":
				add("package.json")
				break
			default:
				console.log(`No such add shortcut: (${arg})`)
		}
	}
} else if (args[0] === "copy") {
	if (args.length === 1) {
		console.log("No shortcut arguements passed")
	}
	for (let i = 1, il = args.length; i < il; i++) {
		const arg = args[i]
		switch (arg) {
			case "debug":
				copy("debug.json")
				break
			case "app":
				copy("app.js")
				break
			case "build-apk":
				copy("build-apk.txt")
				break
			default:
				console.log(`No such copy shortcut: (${arg})`)
		}
	}
} else {
	console.log(`No such command (${args[0]})`)
}
