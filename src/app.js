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
					fs.copyFile(path.join(__dirname, "../add", file + ".bkp"), file, () =>
						console.log(`Wrote to ${file}`)
					)
				})
			}
		}
	}

	if (args[0] === "copy") {
		if (args.length === 1) {
			console.log("No files arguements passed")
			console.log(`Existing operations: ${Object.keys(mappings.copy).join(", ")}`)
			process.exit()
		}

		for (let i = 1, il = args.length; i < il; i++) {
			if (args[i] in mappings.copy) {
				mappings.copy[args[i]].forEach(file => {
					console.log(
						fs.readFileSync(path.join(__dirname, "../copy", file + ".bkp"), "utf-8")
					)
				})
			}
		}
	}

	if (args[0] === "gen") {
		if (args.length === 1) {
			console.log("No file arguement passed")
			process.exit()
		}

		if (args.length > 2) {
			console.log("Too many arguements passed")
			process.exit()
		}

		fs.readFile(args[1] + "/package.json", (err, data) => {
			if (err) {
				console.error("Could not find a package.json in that folder")
				process.exit()
			}

			const repository = process.cwd().split("\\").at(-1)
			const json = JSON.parse(data)

			for (const dependency in json.dependencies) {
				console.log(
					`\t-   [![${dependency}](https://img.shields.io/github/package-json/dependency-version/zS1L3NT/${repository}/${dependency}?style=flat-square${
						args[1] !== "." ? `&filename=${args[1]}/package.json` : ""
					})](https://npmjs.com/package/${dependency})`
				)
			}

			for (const dependency in json.devDependencies || {}) {
				console.log(
					`\t-   [![${dependency}](https://img.shields.io/github/package-json/dependency-version/zS1L3NT/${repository}/dev/${dependency}?style=flat-square${
						args[1] !== "." ? `&filename=${args[1]}/package.json` : ""
					})](https://npmjs.com/package/${dependency})`
				)
			}
		})
	}
} else {
	console.log(`No such operation  : ${args[0]}`)
	console.log(`Existing operations: ${Object.keys(mappings).join(", ")}`)
}
