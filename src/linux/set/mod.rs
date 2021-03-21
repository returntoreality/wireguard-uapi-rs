mod allowed_ip;
pub use allowed_ip::AllowedIp;
mod device;
pub use device::Device;
pub use device::NetlinkDevice;
mod peer;
pub use peer::Peer;

mod create_set_device_messages;
pub(crate) use create_set_device_messages::create_set_device_messages;

pub const WGDEVICE_F_REPLACE_PEERS: u32 = 1 << 0;
pub const WGPEER_F_REMOVE_ME: u32 = 1 << 0;
pub const WGPEER_F_REPLACE_ALLOWEDIPS: u32 = 1 << 1;
pub const WGPEER_F_UPDATE_ONLY: u32 = 1 << 2;
