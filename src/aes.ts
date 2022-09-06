import { AES } from "https://deno.land/x/god_crypto@v1.4.10/aes.ts"

export const encrypt = async (data: string, key: string) => {
	const aes = new AES(key.repeat(32).slice(0, 32), {
		mode: "cbc",
		iv: Deno.env.get("AES__IV")
	})

	return (await aes.encrypt(data)).base64()
}

export const decrypt = async (data: string, key: string) => {
	const aes = new AES(key.repeat(32).slice(0, 32), {
		mode: "cbc",
		iv: Deno.env.get("AES__IV")
	})

	return (
		await aes.decrypt(
			new Uint8Array(
				atob(data)
					.split("")
					.map(c => c.charCodeAt(0))
			)
		)
	).toString()
}
