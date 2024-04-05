//! Cloudflare API client. Includes structs that the API responds with and a helper to make requests to the API.

use std::fmt;

use serde::{Deserialize, Serialize};

pub trait CloudflareAuthorizer: fmt::Debug {
    fn with_auth(&self, request_builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder;
}

#[derive(Debug)]
pub struct BearerAuthorizer {
    pub token: String,
}

#[derive(Debug)]
pub struct ApiKeyAuthorizer {
    pub key: String,
}

impl CloudflareAuthorizer for BearerAuthorizer {
    fn with_auth(&self, request_builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        request_builder.bearer_auth(&self.token)
    }
}

impl CloudflareAuthorizer for ApiKeyAuthorizer {
    fn with_auth(&self, request_builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        request_builder.header("X-Auth-Key", &self.key)
    }
}

/// Cloudflare serializes errors and messages the same way.
///
/// - code: The error code. An integer >= 1000.
/// - message: A human-readable message.
#[derive(Debug, Deserialize, Serialize)]
pub struct CloudflareMessage {
    /// The error or message code. An integer >= 1000.
    pub code: u32,
    /// A human-readable message.
    pub message: String,
}

/// Cloudflare API responses include result information sometimes.
#[derive(Debug, Deserialize, Serialize)]
pub struct CloudflareResultInfo {
    /// The page number of the current page.
    pub page: u32,
    /// The number of items per page.
    pub per_page: u32,
    /// The total number of pages.
    pub total_pages: u32,
    /// The total number of items.
    pub count: u32,
    /// The total number of items that would be available without any filtering.
    pub total_count: u32,
}

/// A Cloudflare API response.
#[derive(Debug, Deserialize, Serialize)]
pub struct CloudflareResponse<T> {
    /// The result of the API call.
    pub result: T,
    /// Whether the API call was successful.
    pub success: bool,
    /// Errors returned by the API.
    pub errors: Vec<CloudflareMessage>,
    /// Messages returned by the API.
    pub messages: Vec<CloudflareMessage>,
}

/// A Cloudflare API response.
#[derive(Debug, Deserialize, Serialize)]
pub struct CloudflareResponseWithInfo<T> {
    /// The result of the API call.
    pub result: T,
    /// Whether the API call was successful.
    pub success: bool,
    /// Errors returned by the API.
    pub errors: Vec<CloudflareMessage>,
    /// Messages returned by the API.
    pub messages: Vec<CloudflareMessage>,
    /// Result information returned by the API. (Optional)
    pub result_info: CloudflareResultInfo,
}

/// Cloudflare account information.
#[derive(Debug, Deserialize, Serialize)]
pub struct CloudflareAccount {
    /// The account identifier.
    pub id: String,
    /// The account name.
    pub name: String,
}

/// Cloudflare zone owner information.
#[derive(Debug, Deserialize, Serialize)]
pub struct CloudflareOwner {
    /// The owner ID.
    ///
    /// Example: `023e105f4ecef8ad9ca31a8372d0c353`
    pub id: Option<String>,
    /// The name of the owner.
    ///
    /// Example: `Cloudflare, Inc.`
    pub name: Option<String>,
    /// The type of the owner.
    ///
    /// Example: `organization`
    #[serde(rename = "type")]
    pub type_: String,
}

/// Cloudflare zone metadata information.
#[derive(Debug, Deserialize, Serialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct CloudflareZoneMetadata {
    /// The zone is only configured for CDN
    pub cdn_only: bool,
    /// Number of Custom Certificates the zone can have
    pub custom_certificate_quota: i32,
    /// The zone is only configured for DNS
    pub dns_only: bool,
    /// The zone is setup with Foundation DNS
    pub foundation_dns: bool,
    /// Number of Page Rules a zone can have
    pub page_rule_quota: i32,
    /// The zone has been flagged for phishing
    pub phishing_detected: bool,
    /// Not described in documentation, example value: `2`
    pub step: i32,
}

/// Cloudflare endpoint response for List Zones.
#[derive(Debug, Deserialize, Serialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct CloudflareListZonesResponse {
    /// The account the zone belongs to.
    pub account: CloudflareAccount,
    /// The last time proof of ownership was detected and the zone was made active
    ///
    /// Example: `2014-01-02T00:01:00.12345Z`
    pub activated_on: String,
    /// The time the zone was created
    ///
    /// Example: `2014-01-01T05:20:00.12345Z`
    pub created_on: String,
    /// The interval (in seconds) from when development mode expires (positive integer)
    /// or last expired (negative integer) for the domain. If development mode has never been enabled, this value is 0.
    pub development_mode: i32,
    /// The zone identifier
    ///
    /// This is a string <= 32 characters.
    ///
    /// Example: `023e105f4ecef8ad9ca31a8372d0c353`
    pub id: String,
    /// When the zone was last modified
    ///
    /// Example: `2014-01-01T05:20:00.12345Z`
    pub modified_on: String,
    /// The domain name. This is <= 253 characters.
    ///
    /// Example: `example.com`
    ///
    /// Match pattern: `^([a-zA-Z0-9][\-a-zA-Z0-9]*\.)+[\-a-zA-Z0-9]{2,20}$`
    pub name: String,
    /// DNS host at the time of switching to Cloudflare. <= 50 characters.
    ///
    /// Example: `NameCheap`
    pub original_dnshost: Option<String>,
    /// Original name servers before moving to Cloudflare Notes: Is this only available for full zones?
    ///
    /// Example: `["ns1.originaldnshost.com","ns2.originaldnshost.com"]`
    pub original_name_servers: Option<Vec<String>>,
    /// Registrar for the domain at the time of switching to Cloudflare. <= 50 characters.
    ///
    /// Example: `GoDaddy`
    pub original_registrar: Option<String>,
    /// The owner of the zone.
    pub owner: CloudflareOwner,
    /// An array of domains used for custom name servers. This is only available for Business and Enterprise plans.
    ///
    /// Example: `["ns1.example.com","ns2.example.com"]`
    pub vanity_name_servers: Option<Vec<String>>,
    /// What tenant the zone is in
    pub tenant: CloudflareTenant,
    /// What tenant unit the zone is in
    pub tenant_unit: CloudflareTenantUnit,
    /// What permissions are available on the zone
    pub permissions: Vec<String>,
    /// What plan the zone is on
    pub plan: CloudflarePlan,
    /// The zone's status
    pub status: String,
    /// Whether the zone is paused
    pub paused: bool,
}

/// Cloudflare plan information.
#[derive(Debug, Deserialize, Serialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct CloudflarePlan {
    /// The plan identifier
    pub id: String,
    /// The plan name
    pub name: String,
    /// The plan price
    pub price: f64,
    /// The plan currency
    pub currency: String,
    /// The plan frequency
    pub frequency: String,
    /// Is the plan a subscription?
    pub is_subscribed: bool,
    /// Can the plan be a subscription?
    pub can_subscribe: bool,
    /// The plan's legacy identifier
    pub legacy_id: String,
    /// Does the plan have a legacy discount?
    pub legacy_discount: bool,
    /// Is the plan externally managed?
    pub externally_managed: bool,
}

/// Cloudflare tenet unit information.
#[derive(Debug, Deserialize, Serialize)]
pub struct CloudflareTenantUnit {
    /// The tenant unit identifier
    pub id: Option<String>,
}

/// Cloudflare tenet information.
#[derive(Debug, Deserialize, Serialize)]
pub struct CloudflareTenant {
    /// The tenant identifier
    pub id: Option<String>,
    /// The tenant name
    pub name: Option<String>,
}

/// Cloudflare organization information.
#[derive(Debug, Deserialize, Serialize)]
pub struct CloudflareOrganizationUserDetails {
    /// The organization identifier for the user
    pub id: String,
    /// The organization name
    pub name: String,
    /// The user's role in the organization
    pub status: String,
    /// The organization permissions the user has
    pub permissions: Vec<String>,
    /// The organization's roles the user has
    pub roles: Vec<String>,
}

/// Cloudflare endpoint response for Get User Details
#[derive(Debug, Deserialize, Serialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct CloudflareUserDetailsResponse {
    /// The user's identifier
    ///
    /// Example: `023e105f4ecef8ad9ca31a8372d0c353`
    pub id: String,
    /// The user's email address
    pub email: String,
    /// The user's username
    pub username: String,
    /// The user's first name
    pub first_name: Option<String>,
    /// The user's last name
    pub last_name: Option<String>,
    /// The user's telephone number
    pub telephone: Option<String>,
    /// The user's country
    pub country: Option<String>,
    /// The user's zipcode
    pub zipcode: Option<String>,
    /// Does the user have TFA enabled?
    pub two_factor_authentication_enabled: bool,
    /// Is the user's TFA locked?
    pub two_factor_authentication_locked: bool,
    /// The user's created date
    ///
    /// Example: `2014-01-01T05:20:00.12345Z`
    pub created_on: String,
    /// The user's modified date
    ///
    /// Example: `2014-01-01T05:20:00.12345Z`
    pub modified_on: String,
    /// The user's organizations
    pub organizations: Vec<CloudflareOrganizationUserDetails>,
    /// Whether the user has pro zones
    pub has_pro_zones: bool,
    /// Whether the user has business zones
    pub has_business_zones: bool,
    /// Whether the user has enterprise zones
    pub has_enterprise_zones: bool,
    /// Whether the user's account is suspended
    pub suspended: bool,
    /// The user's beta entitlements
    pub betas: Vec<String>,
}
