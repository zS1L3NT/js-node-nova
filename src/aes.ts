import { AES } from "god_crypto/aes.ts"

const createAes = (key: string) =>
	new AES(key.repeat(32).slice(0, 32), {
		mode: "cbc",
		iv: Deno.env.get("AES__IV")
	})

export const validate = async (key: string) => {
	try {
		return key === (await decrypt(Deno.env.get("AES__ENCRYPTED_KEY")!, key))
	} catch {
		return false
	}
}

export const encrypt = async (data: string, key: string) => {
	return (await createAes(key).encrypt(data)).base64()
}

export const decrypt = async (data: string, key: string) => {
	return (
		await createAes(key).decrypt(
			new Uint8Array(
				atob(data)
					.split("")
					.map(c => c.charCodeAt(0))
			)
		)
	).toString()
}
