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

		const dependencies = []
		for (const arg of args.slice(1)) {
			try {
				const json = JSON.parse(fs.readFileSync(arg + "/package.json", "utf-8"))
				dependencies.push(
					...Object.keys(json.dependencies),
					...Object.keys(json.devDependencies || {}).map(d => "#" + d)
				)
			} catch {
				console.error("Could not find a package.json in " + arg)
				process.exit()
			}
		}

		const repository = process.cwd().split("\\").at(-1)
		for (let dependency of dependencies.sort()) {
			const dev = dependency.startsWith("#")
			dependency = dependency.replace("#", "")
			console.log(
				[
					"\t-   [![",
					dependency,
					"](https://img.shields.io/github/package-json/dependency-version/zS1L3NT/",
					repository,
					dev ? "/dev/" : "/",
					dependency,
					"?style=flat-square",
					args[1] !== "." ? `&filename=${args[1]}/package.json` : "",
					")](https://npmjs.com/package/",
					dependency,
					")"
				].join("")
			)
		}
	}
} else {
	console.log(`No such operation  : ${args[0]}`)
	console.log(`Existing operations: ${Object.keys(mappings).join(", ")}`)
}
