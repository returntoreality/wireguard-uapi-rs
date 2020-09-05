use super::socket::GenlSocket;

mod attr;
mod get_family;

#[repr(u8)]
enum ControllerCommand {
    Unspec = libc::CTRL_CMD_UNSPEC as u8,
    NewFamily = libc::CTRL_CMD_NEWFAMILY as u8,
    DelFamily = libc::CTRL_CMD_DELFAMILY as u8,
    GetFamily = libc::CTRL_CMD_GETFAMILY as u8,
    NewOps = libc::CTRL_CMD_NEWOPS as u8,
    DelOps = libc::CTRL_CMD_DELOPS as u8,
    GetOps = libc::CTRL_CMD_GETOPS as u8,
    NewMulticastGroup = libc::CTRL_CMD_NEWMCAST_GRP as u8,
    DelMulticastGroup = libc::CTRL_CMD_DELMCAST_GRP as u8,
    GetMulticastGroup = libc::CTRL_CMD_GETMCAST_GRP as u8,
}

pub trait NetlinkGenericController {
    fn get_family(&self, family_name: &str) -> nix::Result<u16>;
}

impl NetlinkGenericController for GenlSocket {
    fn get_family(&self, family_name: &str) -> nix::Result<u16> {
        get_family::get_family(&self, family_name)
    }
}
