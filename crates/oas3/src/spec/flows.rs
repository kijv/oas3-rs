use std::{collections::BTreeMap, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize};
use url::Url;

use super::spec_extensions;

/// Allows configuration of the supported OAuth Flows.
///
/// See <https://spec.openapis.org/oas/v3.1.1#oauth-flows-object>.
#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Flows {
    /// Configuration for the OAuth Implicit flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit: Option<ImplicitFlow>,

    /// Configuration for the OAuth Resource Owner Password flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<PasswordFlow>,

    /// Configuration for the OAuth Client Credentials flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_credentials: Option<ClientCredentialsFlow>,

    /// Configuration for the OAuth Authorization Code flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<AuthorizationCodeFlow>,

    /// Specification extensions.
    ///
    /// Only "x-" prefixed keys are collected, and the prefix is stripped.
    ///
    /// See <https://spec.openapis.org/oas/v3.1.1#specification-extensions>.
    #[serde(flatten, with = "spec_extensions")]
    pub extensions: BTreeMap<String, serde_json::Value>,
}

/// Configuration details for a implicit OAuth Flow.
///
/// See <https://spec.openapis.org/oas/v3.1.1#oauth-flow-object>.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImplicitFlow {
    /// The authorization URL to be used for this flow.
    ///
    /// This MUST be in the form of a URL. The OAuth2 standard requires the use of TLS.
    pub authorization_url: Url,

    /// The URL to be used for obtaining refresh tokens.
    ///
    /// This MUST be in the form of a URL. The OAuth2 standard requires the use of TLS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<Url>,

    /// The available scopes for the OAuth2 security scheme.
    ///
    /// A map between the scope name and a short description for it. The map MAY be empty.
    #[serde(default)]
    pub scopes: BTreeMap<String, String>,

    /// Specification extensions.
    ///
    /// Only "x-" prefixed keys are collected, and the prefix is stripped.
    ///
    /// See <https://spec.openapis.org/oas/v3.1.1#specification-extensions>.
    #[serde(flatten, with = "spec_extensions")]
    pub extensions: BTreeMap<String, serde_json::Value>,
}

/// Configuration details for a password OAuth Flow.
///
/// See <https://spec.openapis.org/oas/v3.1.1#oauth-flow-object>.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordFlow {
    /// The token URL to be used for this flow.
    ///
    /// This MUST be in the form of a URL. The OAuth2 standard requires the use of TLS.
    pub token_url: UrlWithRelativeUrlAsFileUrl,

    /// The URL to be used for obtaining refresh tokens.
    ///
    /// This MUST be in the form of a URL. The OAuth2 standard requires the use of TLS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<Url>,

    /// The available scopes for the OAuth2 security scheme.
    ///
    /// A map between the scope name and a short description for it. The map MAY be empty.
    #[serde(default)]
    pub scopes: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct UrlWithRelativeUrlAsFileUrl(Url);

impl<'de> Deserialize<'de> for UrlWithRelativeUrlAsFileUrl {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>
  {
    let str = String::deserialize(deserializer)?;

    Ok(
      UrlWithRelativeUrlAsFileUrl(
        if str.starts_with('/') {
          Url::from_file_path(str).unwrap()
        } else {
          Url::parse(&str).unwrap()
        }
      )
    )
  }
}

/// Configuration details for a client credentials OAuth Flow.
///
/// See <https://spec.openapis.org/oas/v3.1.1#oauth-flow-object>.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientCredentialsFlow {
    /// The token URL to be used for this flow.
    ///
    /// This MUST be in the form of a URL. The OAuth2 standard requires the use of TLS.
    pub token_url: UrlWithRelativeUrlAsFileUrl,

    /// The URL to be used for obtaining refresh tokens.
    ///
    /// This MUST be in the form of a URL. The OAuth2 standard requires the use of TLS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<Url>,

    /// The available scopes for the OAuth2 security scheme.
    ///
    /// A map between the scope name and a short description for it. The map MAY be empty.
    #[serde(default)]
    pub scopes: BTreeMap<String, String>,
}

/// Configuration details for a authorization code OAuth Flow.
///
/// See <https://spec.openapis.org/oas/v3.1.1#oauth-flow-object>.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizationCodeFlow {
    /// The authorization URL to be used for this flow.
    ///
    /// This MUST be in the form of a URL. The OAuth2 standard requires the use of TLS.
    pub authorization_url: Url,

    /// The token URL to be used for this flow.
    ///
    /// This MUST be in the form of a URL. The OAuth2 standard requires the use of TLS.
    pub token_url: UrlWithRelativeUrlAsFileUrl,

    /// The URL to be used for obtaining refresh tokens.
    ///
    /// This MUST be in the form of a URL. The OAuth2 standard requires the use of TLS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<Url>,

    /// The available scopes for the OAuth2 security scheme.
    ///
    /// A map between the scope name and a short description for it. The map MAY be empty.
    #[serde(default)]
    pub scopes: BTreeMap<String, String>,
}
