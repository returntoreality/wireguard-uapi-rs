mod route_socket;
pub use route_socket::RouteSocket;

mod wg_socket;
pub use wg_socket::WgSocket;

#[cfg(feature = "xplatform")]
mod xplatform_linux_client;
#[cfg(feature = "xplatform")]
pub use xplatform_linux_client::CrossPlatformLinuxClient;

pub(crate) mod parse;

pub(crate) type NlWgMsgType = u16;

pub(crate) mod link_message;
pub(crate) use link_message::{link_message, WireGuardDeviceLinkOperation};

pub(crate) mod list_device_names_utils;
