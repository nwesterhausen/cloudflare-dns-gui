import { createContext, type ParentComponent, useContext, createSignal, createEffect } from "solid-js";
import { CLOUDFLARE_API_KEY } from "../lib";
import { invoke } from "@tauri-apps/api/core";

type TokenContextData = [
	apiToken: () => string,
	apiReady: () => boolean,
	{ setApiToken: (newToken: string) => void; resetApi: () => void },
];

const TokenContext = createContext<TokenContextData>([
	() => "",
	() => false,
	{
		setApiToken: () => {},
		resetApi: () => {},
	},
]);

export const TokenProvider: ParentComponent = (props) => {
	const [apiToken, _setApiToken] = createSignal<string>("");
	const [apiReady, _setApiReady] = createSignal<boolean>(false);
	const resetApi = () => {
		_setApiToken("");
	};
	const setApiToken = (newToken: string) => {
		_setApiToken(newToken);
	};

	setTimeout(async () => {
		_setApiToken(localStorage.getItem(CLOUDFLARE_API_KEY) || "");
		checkSetApiToken();
	}, 10);

	async function checkSetApiToken() {
		localStorage.setItem(CLOUDFLARE_API_KEY, apiToken());
		_setApiReady(false);
		console.log("Setting CF token");
		await invoke("set_api_token", { token: apiToken() });
		console.log("Initializing CF cache");
		const ready = await invoke("initialize_cf", {});
		if (!ready) {
			console.error("Failed to initialize CF cache");
			_setApiReady(false);
			_setApiToken("");
			return;
		}
		console.log("CF cache initialized");
		_setApiReady(true);
	}

	createEffect(async () => {
		if (apiToken().length === 0) {
			_setApiReady(false);
			localStorage.removeItem(CLOUDFLARE_API_KEY);
			return;
		}
		checkSetApiToken();
	});

	const contextData: TokenContextData = [apiToken, apiReady, { setApiToken, resetApi }];

	return <TokenContext.Provider value={contextData}>{props.children}</TokenContext.Provider>;
};

export const useTokenProvider = () => useContext(TokenContext);
