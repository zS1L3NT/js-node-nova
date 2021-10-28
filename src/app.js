const args = process.argv.slice(2)
const path = require("path")
const fs = require("fs")

const mappings = require("./mappings.json")

if (args.length === 0) {
	console.log(`No operation arguements passed`)
	console.log(`Existing operations: ${Object.keys(mappings).join(", ")}`)
	process.exit()
}

if (args[0] in mappings) {
	if (args[0] === "add") {
		if (args.length === 1) {
			console.log("No files arguements passed")
			console.log(`Existing operations: ${Object.keys(mappings.add).join(", ")}`)
			process.exit()
		}

		for (let i = 1, il = args.length; i < il; i++) {
			if (args[i] in mappings.add) {
				mappings.add[args[i]].forEach(file => {
					fs.copyFile(path.join(__dirname, "add", file + ".bkp"), file, () => console.log(`Wrote to ${file}`))
				})
			}
		}
	}

	if (args[0] === "copy") {
		if (args.length === 1) {
			console.log("No files arguements passed")
			console.log(`Existing operations: ${Object.keys(mappings.copy).join(", ")}`)
		}

		for (let i = 1, il = args.length; i < il; i++) {
			if (args[i] in mappings.copy) {
				mappings.copy[args[i]].forEach(file => {
					console.log(fs.readFileSync(path.join(__dirname, "copy", file), "utf-8"))
				})
			}
		}
	}
} else {
	console.log(`No such operation  : ${args[0]}`)
	console.log(`Existing operations: ${Object.keys(mappings).join(", ")}`)
}
