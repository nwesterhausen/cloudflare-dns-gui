// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { CloudflareMessage } from "./CloudflareMessage";

/**
 * A Cloudflare API response.
 */
export type CloudflareResponse<T> = { 
/**
 * The result of the API call.
 */
result: T, 
/**
 * Whether the API call was successful.
 */
success: boolean, 
/**
 * Errors returned by the API.
 */
errors: Array<CloudflareMessage>, 
/**
 * Messages returned by the API.
 */
messages: Array<CloudflareMessage>, };