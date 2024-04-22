import { createSignal } from "solid-js";
import Lib from "./lib";
import { invoke } from "@tauri-apps/api/core";

function Navbar() {
	const [statusButtonText, setStatusButtonText] = createSignal(statusButtonTextGen());
	function successIconGen() {
		if (localStorage.getItem(Lib.API_READY)) return "✅";
		return "🙅";
	}
	function statusButtonTextGen() {
		return `API: ${localStorage.getItem(Lib.CLOUDFLARE_API_KEY) ? successIconGen() : "❌"}`;
	}

	// Update the token signal when the localStorage changes
	window.addEventListener("storage", () => {
		setStatusButtonText(statusButtonTextGen());
	});

	return (
		<div class="navbar bg-base-100">
			<div class="navbar-start">
				<span class="btn btn-ghost text-xl">DNS Tool</span>
			</div>
			<div class="navbar-center">
				<button
					type="button"
					class="btn btn-ghost text-xl"
					onClick={async () => {
						if (localStorage.getItem(Lib.CLOUDFLARE_API_KEY)) {
							localStorage.removeItem(Lib.CLOUDFLARE_API_KEY);
							localStorage.removeItem(Lib.API_READY);
						} else {
							const key = prompt("Enter your Cloudflare API key:");
							if (!key) return;
							localStorage.setItem(Lib.CLOUDFLARE_API_KEY, key);
							console.log("Setting API token");
							await invoke("set_api_token", { token: key });
							console.log("Initializing CF cache");
							localStorage.setItem(Lib.API_READY, await invoke("initialize_cf", {}));
						}

						// Force re-render
						window.location.reload();
					}}
				>
					{statusButtonText()}
				</button>
			</div>
			<div class="navbar-end">
				<div class="flex-none">
					<button type="button" class="btn btn-square btn-ghost">
						<svg
							xmlns="http://www.w3.org/2000/svg"
							fill="none"
							viewBox="0 0 24 24"
							class="inline-block w-5 h-5 stroke-current"
						>
							<title>hamburger menu</title>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M5 12h.01M12 12h.01M19 12h.01M6 12a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0z"
							/>
						</svg>
					</button>
				</div>
			</div>
		</div>
	);
}

export default Navbar;
