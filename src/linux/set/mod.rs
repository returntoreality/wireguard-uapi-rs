mod allowed_ip;
mod device;
pub(crate) use device::NetlinkDevice;

mod create_set_device_messages;
pub(crate) use create_set_device_messages::create_set_device_messages;

pub const WGDEVICE_F_REPLACE_PEERS: u32 = 1 << 0;
pub const WGPEER_F_REMOVE_ME: u32 = 1 << 0;
pub const WGPEER_F_REPLACE_ALLOWEDIPS: u32 = 1 << 1;
pub const WGPEER_F_UPDATE_ONLY: u32 = 1 << 2;
