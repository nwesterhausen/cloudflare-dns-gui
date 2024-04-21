import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import Navbar from "./Navbar";

function App() {
  const [greetMsg, setGreetMsg] = createSignal(<div />);
  const token = () => localStorage.getItem("CF-APIKEY");

  async function check_api_key() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    const response = await invoke("check_api_key", { token: token() });
    if (response) {
      setGreetMsg(<div class="flex flex-col gap-3"><div>API Key is valid.</div><div>User: {response.email}</div><div>Organizations: {response.organizations.join(", ")}</div>
      </div>);
    }
  }

  async function get_zones() {
    setGreetMsg(
    <div class="flex flex-col items-center gap-3 my-2 w-96 h-48">
      <div class="my-2 px-2 text-white">📡 Communicating with Cloudflare</div>
      <div class="flex flex-row items-center grow">
       <div><span class="loading loading-infinity loading-lg" /></div>
       </div>
     </div>);

    const zones = await invoke("get_zones", { token: token() });

    if (zones.success) {
      const zoneDns = {};
      for (const zone of zones.result) {
        zoneDns[zone.id] = await invoke("get_zone_dns", { token: token(), zoneId: zone.id });
        if (zoneDns[zone.id].success) {
          zoneDns[zone.id] = zoneDns[zone.id].result;
        } else {
          zoneDns[zone.id] = [];
        }
      }

      console.log(zoneDns)

      setGreetMsg(<div class="flex flex-col gap-3 my-2 w-96">
      <For each={zones.result}>{
        (zone) => <div class="flex flex-row gap-3 rounded-full border-2 border-indigo-500 bg-slate-900 hover:bg-slate-700 px-4 max-w-md">
          <div class="font-mono font-bold text-lg underline grow">{zone.name}</div>
          <div class="text-cyan-400">{zoneDns[zone.id].length} DNS records</div>
        </div>
      }
      </For>
      </div>);
    }
  }

  return (
    <>
    <Navbar />

<div class="container mx-auto">

      <button type="button" class="btn btn-primary" onClick={check_api_key}>
        Check API Key
      </button>
      <button type="button" class="btn btn-secondary" onClick={get_zones}>
        Get Zones
      </button>
      <br />
      <div>{greetMsg()}</div>
    </div>
  </>
  );
}

export default App;
