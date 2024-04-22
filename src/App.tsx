import { For, Show } from "solid-js";
import Navbar from "./Navbar";
import { Toaster } from "./Toaster";
import { useZonesProvider } from "./providers/ZoneContext";

function App() {
	const zonesCtx = useZonesProvider();
	if (!zonesCtx) return <div>Loading...</div>;
	const [zones, zoneDns] = zonesCtx;

	return (
		<>
			<Navbar />
			<Toaster />

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
			</div>
		</>
	);
}

export default App;
