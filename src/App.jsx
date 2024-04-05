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
    const zones = await invoke("get_zones", { token: token() });

    if (zones.success) {
      setGreetMsg(<div class="flex flex-col gap-3">
      <For each={zones.result}>{
        (zone) => <div>{zone.name}</div>
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
