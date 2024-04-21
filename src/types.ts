import type { DNSRecord } from "../src-tauri/bindings/DNSRecord";

export type CloudflareZoneDnsResponse = {
	[key: string]: DNSRecord[];
};
