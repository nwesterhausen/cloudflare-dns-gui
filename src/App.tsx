import { createEffect, createSignal, For, type JSX, Show } from "solid-js";
import Navbar from "./Navbar";
import { useTokenProvider } from "./TokenProvider";
import { useZone } from "./ZoneProvider";

function App() {
	const { apiToken, userDetails } = useTokenProvider();
	const { zones, zoneDns } = useZone();

	const [toast, setToast] = createSignal<JSX.Element>(<div />);
	const putToast = (token: JSX.Element) => {
		setToast(token);
		setTimeout(() => {
			setToast(<div />);
		}, 5_000);
	};

	createEffect(() => {
		if (apiToken().length > 0 && userDetails.latest.email.length > 0) {
			putToast(
				<div class="alert alert-success">
					<div class="flex flex-col gap-3">
						<div>API Key is valid.</div>
						<div>User: {userDetails.latest.email}</div>
						<div>Organizations: {userDetails.latest.organizations.join(", ")}</div>
					</div>
				</div>,
			);
		}
	});

	return (
		<>
			<Navbar />

			<div class="container mx-auto">
				<div>
					<div class="flex flex-col gap-3 my-2 w-96">
						<For each={zones.latest}>
							{(zone) => (
								<div class="flex flex-row gap-3 rounded-full border-2 border-indigo-500 bg-slate-900 hover:bg-slate-700 px-4 max-w-md">
									<div class="font-mono font-bold text-lg underline grow">{zone.name}</div>
									<Show when={zoneDns.latest[zone.id]} fallback={<div class="text-rose-700">Cache miss?</div>}>
										<div class="text-cyan-400">{zoneDns.latest[zone.id].length} DNS records</div>
									</Show>
								</div>
							)}
						</For>
					</div>
				</div>
				<div class="toast toast-top toast-center">{toast()}</div>
			</div>
		</>
	);
}

export default App;
