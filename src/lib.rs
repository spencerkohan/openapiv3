#![cfg_attr(docsrs, feature(doc_cfg))]
mod callback;
mod components;
mod contact;
mod discriminator;
mod encoding;
mod example;
mod external_documentation;
mod header;
mod info;
mod license;
mod link;
mod media_type;
mod openapi;
mod operation;
mod parameter;
mod paths;
mod reference;
mod request_body;
mod responses;
mod schema;
mod security_requirement;
mod security_scheme;
mod server;
mod server_variable;
mod status_code;
mod tag;
mod util;
mod variant_or;
#[cfg(feature = "v2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2")))]
pub mod v2;
mod versioned;
mod map;

pub use self::callback::*;
pub use self::components::*;
pub use self::contact::*;
pub use self::discriminator::*;
pub use self::encoding::*;
pub use self::example::*;
pub use self::external_documentation::*;
pub use self::header::*;
pub use self::info::*;
pub use self::license::*;
pub use self::link::*;
pub use self::media_type::*;
pub use self::openapi::*;
pub use self::operation::*;
pub use self::parameter::*;
pub use self::paths::*;
pub use self::reference::*;
pub use self::request_body::*;
pub use self::responses::*;
pub use self::schema::*;
pub use self::security_requirement::*;
pub use self::security_scheme::*;
pub use self::server::*;
pub use self::server_variable::*;
pub use self::status_code::*;
pub use self::tag::*;
pub use self::util::*;
pub use self::variant_or::*;
pub use map::*;
pub use http::method::Method as PathMethod;
pub use versioned::*;
pub use indexmap::IndexMap;

fn default<T: Default>() -> T {
    T::default()
}