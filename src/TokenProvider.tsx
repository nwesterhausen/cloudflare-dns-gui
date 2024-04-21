import { createContextProvider } from "@solid-primitives/context";
import { createResource, createSignal } from "solid-js";
import { CLOUDFLARE_API_KEY } from "./lib";
import type { CustomUserDetails } from "../src-tauri/bindings/CustomUserDetails";
import { invoke } from "@tauri-apps/api/core";

const [TokenProvider, useTokenProvider] = createContextProvider(() => {
	const [apiToken, setApiToken] = createSignal<string>("");
	const [userDetails] = createResource<CustomUserDetails, string>(
		apiToken(),
		async () => {
			return (await invoke("get_user_details", {})) as CustomUserDetails;
		},
		{
			initialValue: {
				email: "",
				organizations: [],
				id: "",
				suspended: false,
			},
		},
	);

	setTimeout(() => {
		setApiToken(localStorage.getItem(CLOUDFLARE_API_KEY) || "");
	}, 10);

	// Update the token signal when the localStorage changes
	window.addEventListener("storage", () => {
		setApiToken(localStorage.getItem(CLOUDFLARE_API_KEY) || "");
	});

	return { apiToken, setApiToken, userDetails };
});

export { TokenProvider, useTokenProvider };
