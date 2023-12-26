use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// Defines a security scheme that can be used by the operations.
/// Supported schemes are HTTP authentication, an API key (either as a
/// header or as a query parameter), OAuth2's common flows (implicit, password,
/// application and access code) as defined in RFC6749, and OpenID Connect Discovery.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum SecurityScheme {
    #[serde(rename = "apiKey")]
    APIKey {
        #[serde(rename = "in")]
        location: APIKeyLocation,
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
    #[serde(rename = "http")]
    HTTP {
        // TODO enum. Values recommended (not required) to come from
        // https://www.iana.org/assignments/http-authschemes/http-authschemes.xhtml
        scheme: String,
        #[serde(rename = "bearerFormat")]
        bearer_format: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
    #[serde(rename = "oauth2")]
    OAuth2 {
        flows: OAuth2Flows,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
    #[serde(rename = "openIdConnect")]
    OpenIDConnect {
        #[serde(rename = "openIdConnectUrl")]
        open_id_connect_url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum APIKeyLocation {
    Query,
    Header,
    Cookie,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OAuth2Flows {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub implicit: Option<ImplicitOAuth2Flow>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<OAuth2Flow>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_credentials: Option<OAuth2Flow>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<AuthCodeOAuth2Flow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ImplicitOAuth2Flow {
    pub authorization_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,
    #[serde(default)]
    pub scopes: IndexMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OAuth2Flow {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,
    pub token_url: String,
    #[serde(default)]
    pub scopes: IndexMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AuthCodeOAuth2Flow {
    pub authorization_url: String,
    pub token_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,
    #[serde(default)]
    pub scopes: IndexMap<String, String>,
}