import {
	createContext,
	createEffect,
	createMemo,
	createResource,
	type InitializedResource,
	type ParentComponent,
	useContext,
} from "solid-js";
import type { CloudflareListZonesResponse } from "../../src-tauri/bindings/CloudflareListZonesResponse";
import type { DNSRecord } from "../../src-tauri/bindings/DNSRecord";
import { invoke } from "@tauri-apps/api/core";
import { useTokenProvider } from "./TokenContext";

type ZoneContextData = [
	zones: InitializedResource<CloudflareListZonesResponse[]>,
	zoneDns: InitializedResource<Record<string, DNSRecord[]>>,
	{
		dnsForZone: (zoneId: string) => DNSRecord[];
	},
];

const ZoneContext = createContext<ZoneContextData>();

export const ZoneProvider: ParentComponent = (props) => {
	const [apiToken, apiReady] = useTokenProvider();
	const readyToRun = createMemo(() => apiReady() && apiToken().length > 0);
	const [zoneDns, { refetch: refetchDns }] = createResource<Record<string, DNSRecord[]>>(
		async () => {
			if (!apiReady() || apiToken().length === 0) return {};
			return (await invoke("get_zone_dns", {})) as Record<string, DNSRecord[]>;
		},
		{ initialValue: {} },
	);
	const [zones, { refetch: refetchZones }] = createResource<CloudflareListZonesResponse[]>(
		async () => {
			if (!apiReady() || apiToken().length === 0) return [];
			return (await invoke("list_zones", {})) as CloudflareListZonesResponse[];
		},
		{ initialValue: [] },
	);

	function dnsForZone(zoneId: string) {
		return zoneDns.latest[zoneId] || [];
	}

	createEffect(() => {
		if (readyToRun()) {
			refetchDns();
			refetchZones();
		}
	});

	const contextData: ZoneContextData = [zones, zoneDns, { dnsForZone }];

	return <ZoneContext.Provider value={contextData}>{props.children}</ZoneContext.Provider>;
};

export const useZonesProvider = () => useContext(ZoneContext);
