import { createSignal, For, Show } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import Navbar from "./Navbar";
import { CLOUDFLARE_API_KEY } from "./lib";
import type { CloudflareListZonesResponse } from "../src-tauri/bindings/CloudflareListZonesResponse";
import type { CloudflareZoneDnsResponse } from "./types";
import type { CustomUserDetails } from "../src-tauri/bindings/CustomUserDetails";

function App() {
	const [greetMsg, setGreetMsg] = createSignal(<div />);
	const [token, setToken] = createSignal(localStorage.getItem(CLOUDFLARE_API_KEY));

	// Update the token signal when the localStorage changes
	window.addEventListener("storage", () => {
		setToken(localStorage.getItem(CLOUDFLARE_API_KEY));
	});

	function setLoading() {
		setGreetMsg(
			<div class="flex flex-col items-center gap-3 my-2 w-96 h-48">
				<div class="my-2 px-2 text-white">📡 Communicating with Cloudflare</div>
				<div class="flex flex-row items-center grow">
					<div>
						<span class="loading loading-infinity loading-lg" />
					</div>
				</div>
			</div>,
		);
	}

	async function get_user_details() {
		setLoading();
		// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
		const response = (await invoke("get_user_details", {})) as CustomUserDetails;
		if (response) {
			setGreetMsg(
				<div class="flex flex-col gap-3">
					<div>API Key is valid.</div>
					<div>User: {response.email}</div>
					<div>Organizations: {response.organizations.join(", ")}</div>
				</div>,
			);
			await invoke("initialize_cf", {});
		}
	}

	async function get_zones() {
		setLoading();

		const zones = (await invoke("get_zones", {})) as CloudflareListZonesResponse[];
		const zoneDns = (await invoke("get_zone_dns", {})) as CloudflareZoneDnsResponse;

		setGreetMsg(
			<div class="flex flex-col gap-3 my-2 w-96">
				<For each={zones}>
					{(zone) => (
						<div class="flex flex-row gap-3 rounded-full border-2 border-indigo-500 bg-slate-900 hover:bg-slate-700 px-4 max-w-md">
							<div class="font-mono font-bold text-lg underline grow">{zone.name}</div>
							<Show when={zoneDns[zone.id]} fallback={<div class="text-rose-700">Cache miss?</div>}>
								<div class="text-cyan-400">{zoneDns[zone.id].length} DNS records</div>
							</Show>
						</div>
					)}
				</For>
			</div>,
		);
	}

	return (
		<>
			<Navbar />

			<div class="container mx-auto">
				<button type="button" class="btn btn-primary" disabled={!token()} onClick={get_user_details}>
					Check API Key
				</button>
				<button type="button" class="btn btn-secondary" disabled={!token()} onClick={get_zones}>
					Get Zones
				</button>
				<br />
				<div>{greetMsg()}</div>
			</div>
		</>
	);
}

export default App;
