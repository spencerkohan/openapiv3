use crate::*;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use crate::map::RefOrMap;

/// Holds a set of reusable objects for different aspects of the OAS.
/// All objects defined within the components object will have no effect
/// on the API unless they are explicitly referenced from properties
/// outside the components object.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Components {
    /// An object to hold reusable Security Scheme Objects.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub security_schemes: RefOrMap<SecurityScheme>,
    /// An object to hold reusable Response Objects.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub responses: RefOrMap<Response>,
    /// An object to hold reusable Parameter Objects.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub parameters: RefOrMap<Parameter>,
    /// An object to hold reusable Example Objects.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub examples: RefOrMap<Example>,
    /// An object to hold reusable Request Body Objects.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub request_bodies: RefOrMap<RequestBody>,
    /// An object to hold reusable Header Objects.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub headers: RefOrMap<Header>,
    /// An object to hold reusable Schema Objects.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub schemas: RefOrMap<Schema>,
    /// An object to hold reusable Link Objects.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub links: RefOrMap<Link>,
    /// An object to hold reusable Callback Objects.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub callbacks: RefOrMap<Callback>,
    /// Inline extensions to this object.
    #[serde(flatten, deserialize_with = "crate::util::deserialize_extensions")]
    pub extensions: IndexMap<String, serde_json::Value>,
}

impl Components {
    pub fn is_empty(&self) -> bool {
        self.security_schemes.is_empty()
            && self.responses.is_empty()
            && self.parameters.is_empty()
            && self.examples.is_empty()
            && self.request_bodies.is_empty()
            && self.headers.is_empty()
            && self.schemas.is_empty()
            && self.links.is_empty()
            && self.callbacks.is_empty()
            && self.extensions.is_empty()
    }
}