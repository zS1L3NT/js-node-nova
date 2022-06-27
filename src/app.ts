import * as path from "https://deno.land/std@0.145.0/path/mod.ts"

const getRelativeFile = (file: string) =>
	path.join(path.dirname(path.fromFileUrl(import.meta.url)), file)

const mappings = JSON.parse(await Deno.readTextFile(getRelativeFile("mappings.json")))

if (Deno.args.length === 0) {
	console.log(`No operation arguments passed`)
	console.log(`Existing operations: ${Object.keys(mappings).join(", ")}`)
	Deno.exit()
}

if (Deno.args[0] in mappings) {
	if (Deno.args[0] === "add") {
		if (Deno.args.length === 1) {
			console.log("No files arguments passed")
			console.log(`Existing operations: ${Object.keys(mappings.add).join(", ")}`)
			Deno.exit()
		}

		for (let i = 1, il = Deno.args.length; i < il; i++) {
			if (Deno.args[i] in mappings.add) {
				const file = mappings.add[Deno.args[i]]
				Deno.copyFileSync(getRelativeFile(`../add/${file}.bkp`), mappings.add[Deno.args[i]])
				console.log(`Wrote to ${file}`)
			}
		}
	}

	if (Deno.args[0] === "copy") {
		if (Deno.args.length === 1) {
			console.log("No files arguments passed")
			console.log(`Existing operations: ${Object.keys(mappings.copy).join(", ")}`)
			Deno.exit()
		}

		for (let i = 1, il = Deno.args.length; i < il; i++) {
			if (Deno.args[i] in mappings.copy) {
				const file = mappings.copy[Deno.args[i]]
				console.log(await Deno.readTextFile(getRelativeFile(`../copy/${file}.bkp`)))
			}
		}
	}

	if (Deno.args[0] === "gen") {
		if (Deno.args.length === 1) {
			console.log("No file argument passed")
			Deno.exit()
		}

		if (Deno.args.length > 2) {
			console.log("Too many arguments passed")
			Deno.exit()
		}

		try {
			console.log(Deno.args)
			const data = await Deno.readTextFile(Deno.args[1] + "/package.json")

			const repository = Deno.cwd().split("\\").at(-1)
			const json = JSON.parse(data)
			json.devDependencies = json.devDependencies || {}

			const dependencies = { ...json.dependencies, ...json.devDependencies }
			const sortedDependencies = Object.keys(dependencies)
				.sort()
				.reduce((r, k) => ((r[k] = dependencies[k]), r), {} as Record<string, string>)

			for (const dependency in sortedDependencies) {
				console.log(
					[
						"\t-   [![",
						dependency,
						"](https://img.shields.io/github/package-json/dependency-version/zS1L3NT/",
						repository,
						dependency in json.dependencies ? "/" : "/dev/",
						dependency,
						"?style=flat-square",
						Deno.args[1] !== "." ? `&filename=${Deno.args[1]}/package.json` : "",
						")](https://npmjs.com/package/",
						dependency,
						")"
					].join("")
				)
			}
		} catch (err) {
			console.error("Could not find a package.json in that folder")
			Deno.exit()
		}
	}
} else {
	console.log(`No such operation  : ${Deno.args[0]}`)
	console.log(`Existing operations: ${Object.keys(mappings).join(", ")}`)
}
