import { createContextProvider } from "@solid-primitives/context";
import type { CloudflareZoneDnsResponse } from "./types";
import type { CloudflareListZonesResponse } from "../src-tauri/bindings/CloudflareListZonesResponse";
import { invoke } from "@tauri-apps/api/core";
import { createResource } from "solid-js";
import { useTokenProvider } from "./TokenProvider";

const [ZoneProvider, useZone] = createContextProvider(() => {
	const { apiToken } = useTokenProvider();
	const [zones, { refetch: refetchZones }] = createResource<CloudflareListZonesResponse[], string>(
		apiToken(),
		async () => {
			return (await invoke("get_zones", {})) as CloudflareListZonesResponse[];
		},
		{ initialValue: [] },
	);
	const [zoneDns, { refetch: refetchZoneDns }] = createResource<CloudflareZoneDnsResponse, string>(
		apiToken(),
		async () => {
			return (await invoke("get_zone_dns", {})) as CloudflareZoneDnsResponse;
		},
		{ initialValue: {} },
	);

	return { zones, zoneDns, refetchZones, refetchZoneDns };
});

export { ZoneProvider, useZone };
