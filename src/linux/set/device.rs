use crate::linux::DeviceInterface;

/// The WireGuard Generic Netlink Protocol contains interface name/index fields the cross-platform
/// protocol does not. This type serves as an internal adapter for this difference.
#[derive(Debug)]
pub struct NetlinkDevice<'a> {
    pub interface: DeviceInterface<'a>,
    pub settings: crate::xplatform::set::Device<'a>,
}

pub type Device<'a> = crate::xplatform::set::Device<'a>;
