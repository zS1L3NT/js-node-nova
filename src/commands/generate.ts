// deno-lint-ignore-file no-explicit-any

import { Command } from "cliffy/command/mod.ts"
import path from "path"
import * as YAML from "yaml"

export default new Command()
	.name("generate")
	.description("Generate the `Built with` section for my README.md files")
	.arguments("[directory]")
	.action(async (_, directory) => {
		let errors = 0

		if (!directory) {
			directory = "."
		}

		const project = Deno.cwd().split("\\").at(-1)!

		try {
			const json = JSON.parse(await Deno.readTextFile(directory + "/package.json"))
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
						project,
						dependency in json.dependencies ? "/" : "/dev/",
						dependency,
						"?style=flat-square",
						directory !== "." ? `&filename=${directory}/package.json` : "",
						")](https://npmjs.com/package/",
						dependency,
						")"
					].join("")
				)
			}
		} catch {
			errors++
		}

		try {
			const readDependencies = async (folder: string) => {
				const yaml = <any>(
					YAML.parse(await Deno.readTextFile(path.join(folder, "/pubspec.yaml")))
				)
				yaml.dev_dependencies = yaml.dev_dependencies || {}

				return <Record<string, any>>{
					...yaml.dependencies,
					...yaml.dev_dependencies
				}
			}

			const dependencies = await Promise.all(
				Object.entries(await readDependencies(directory)).map<
					Promise<Record<string, string>>
				>(async ([dependency, value]) =>
					typeof value === "string"
						? { [dependency]: value + "" }
						: "sdk" in value
						? { [dependency]: "sdk" }
						: <Record<string, string>>(
								Object.fromEntries(
									Object.entries(
										await readDependencies(
											path.join(directory!, <string>value.path)
										)
									).filter(([, value]) => typeof value === "string")
								)
						  )
				)
			)

			const sortedDependencies = Object.entries(
				dependencies.reduce(
					(acc, curr) => ({ ...acc, ...curr }),
					<Record<string, string>>{}
				)
			).sort((a, b) => a[0].localeCompare(b[0]))

			for (const [dependency, version] of sortedDependencies) {
				console.log(
					[
						"\t-   [![",
						dependency,
						"](https://img.shields.io/badge/",
						encodeURIComponent(dependency).replaceAll("-", "--"),
						"-",
						encodeURIComponent(version).replaceAll("-", "--"),
						"-blue?style=flat-square",
						")](https://pub.dev/packages/",
						encodeURIComponent(dependency),
						")"
					].join("")
				)
			}
		} catch {
			errors++
		}

		if (errors === 2) {
			console.log("No pubspec.yaml or package.json found")
		}
	})
