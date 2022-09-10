// deno-lint-ignore-file no-explicit-any

import { Command } from "cliffy/command/mod.ts"
import path from "path"
import * as YAML from "yaml"

enum ReadResult {
	NoFile,
	Error,
	Success
}

const readPackageJson = async (directory: string): Promise<ReadResult> => {
	let text: string
	try {
		text = await Deno.readTextFile(path.join(directory, "package.json"))
	} catch {
		return ReadResult.NoFile
	}

	try {
		const package_ = JSON.parse(text)

		if (!("dependencies" in package_)) {
			console.error("No dependencies in package.json, cannot generate dependency list")
			return ReadResult.Error
		}

		package_.devDependencies = package_.devDependencies || {}

		const dependencies = { ...package_.dependencies, ...package_.devDependencies }
		const sortedDependencies = Object.keys(dependencies)
			.sort()
			.reduce((r, k) => ((r[k] = dependencies[k]), r), {} as Record<string, string>)

		for (const dependency in sortedDependencies) {
			console.log(
				[
					"\t-   [![",
					dependency,
					"](https://img.shields.io/github/package-json/dependency-version/zS1L3NT/",
					path.join(Deno.cwd(), directory).split("\\").at(-1)!,
					dependency in package_.dependencies ? "/" : "/dev/",
					dependency,
					"?style=flat-square",
					directory !== "." ? `&filename=${directory}/package.json` : "",
					")](https://npmjs.com/package/",
					dependency,
					")"
				].join("")
			)
		}

		return ReadResult.Success
	} catch (err) {
		console.error(err)
		return ReadResult.Error
	}
}

const readPubspecYaml = async (directory: string): Promise<ReadResult> => {
	try {
		await Deno.readTextFile(path.join(directory, "pubspec.yaml"))
	} catch {
		return ReadResult.NoFile
	}

	try {
		const readDependencies = async (folder: string) => {
			const pubspec = <any>(
				YAML.parse(await Deno.readTextFile(path.join(folder, "pubspec.yaml")))
			)

			if (!("dependencies" in pubspec)) {
				console.error(`No dependencies in ${folder}/pubspec.yaml, cannot generate dependency list`)
				return ReadResult.Error
			}

			pubspec.dev_dependencies = pubspec.dev_dependencies || {}

			return <Record<string, any>>{
				...pubspec.dependencies,
				...pubspec.dev_dependencies
			}
		}

		const dependencies = await Promise.all(
			Object.entries(await readDependencies(directory)).map<Promise<Record<string, string>>>(
				async ([dependency, value]) =>
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
			dependencies.reduce((acc, curr) => ({ ...acc, ...curr }), <Record<string, string>>{})
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

		return ReadResult.Success
	} catch (err) {
		console.error(err)
		return ReadResult.Error
	}
}

const readDenoJson = async (directory: string): Promise<ReadResult> => {
	let text: string
	try {
		text = await Deno.readTextFile(path.join(directory, "deno.json"))
	} catch {
		return ReadResult.NoFile
	}

	try {
		const deno = JSON.parse(text)

		if (!("importMap" in deno)) {
			console.error("No importMap in deno.json, cannot generate dependency list")
			return ReadResult.Error
		}

		const importMap = JSON.parse(await Deno.readTextFile(path.join(directory, deno.importMap)))
		if (!("imports" in importMap)) {
			console.error(`No imports in ${deno.importMap}, cannot generate dependency list`)
			return ReadResult.Error
		}

		const dependencies = Object.entries<string>(importMap.imports)
			.filter(([key]) => key.endsWith("/"))
			.map<[string, string]>(([key, value]) => [
				key.slice(0, -1),
				value.split("@")[1]!.slice(1, -1)
			])

		for (const [dependency, version] of dependencies) {
			console.log(
				[
					"\t-   [![",
					dependency,
					"](https://img.shields.io/badge/",
					encodeURIComponent(dependency).replaceAll("-", "--"),
					"-",
					encodeURIComponent(version).replaceAll("-", "--"),
					"-blue?style=flat-square",
					")](https://deno.land/x/",
					encodeURIComponent(dependency),
					"@v",
					encodeURIComponent(version),
					")"
				].join("")
			)
		}

		return ReadResult.Success
	} catch (err) {
		console.error(err)
		return ReadResult.Error
	}
}

export default new Command()
	.name("generate")
	.description("Generate the `Built with` section for my README.md files")
	.arguments("[directory]")
	.action(async (_, directory) => {
		if (!directory) {
			directory = "."
		}

		const readers = [readPackageJson, readPubspecYaml, readDenoJson]
		for (const reader of readers) {
			const result = await reader(directory)
			if (result !== ReadResult.NoFile) return
		}

		console.log("No package.json, deno.json or pubspec.yaml found")
	})
