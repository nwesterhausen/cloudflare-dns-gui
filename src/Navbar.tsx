import { Match, Switch } from "solid-js";
import { useTokenProvider } from "./providers/TokenContext";

function Navbar() {
	const [apiToken, apiReady, { setApiToken, resetApi }] = useTokenProvider();

	return (
		<div class="navbar bg-base-100">
			<div class="navbar-start">
				<span class="btn btn-ghost text-xl">DNS Tool</span>
			</div>
			<div class="navbar-center">
				<button
					type="button"
					class="btn btn-ghost text-xl"
					onClick={() => {
						if (apiToken().length > 0) {
							resetApi();
							return;
						}

						const key = prompt("Enter your Cloudflare API key:");
						if (!key) return;
						setApiToken(key);
					}}
				>
					{" "}
					API:
					<Switch>
						<Match when={apiToken().length > 0 && apiReady()}>✅</Match>
						<Match when={apiToken().length > 0}>
							📡<span class="loading loading-dots loading-xs" />
						</Match>
						<Match when={true}>❌</Match>
					</Switch>
				</button>
			</div>
		</div>
	);
}

export default Navbar;
