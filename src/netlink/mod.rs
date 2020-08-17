mod linux;
mod types;

use linux::{nlmsg_align, NLMSG_ALIGNTO};
use std::os::unix::io::RawFd;

type Errno = libc::c_int;

fn get_last_errno() -> Errno {
    unsafe { *libc::__errno_location() }
}

fn check_os_error(status: libc::c_int) -> Result<(), Errno> {
    match status {
        -1 => Err(get_last_errno()),
        _ => Ok(()),
    }
}

fn create_genl_socket() -> Result<RawFd, Errno> {
    let domain = libc::AF_NETLINK;
    // From "man 7 netlink":
    //    Netlink is a datagram-oriented service. Both SOCK_RAW and SOCK_DGRAM are
    //    valid values for socket_type. However, the netlink protocol does not
    //    distinguish between datagram and raw sockets.
    let ty = libc::SOCK_RAW;
    let protocol = libc::NETLINK_GENERIC;

    let fd = unsafe { libc::socket(domain, ty, protocol) };
    check_os_error(fd)?;

    Ok(fd)
}

fn create_nl_addr(pid: u32, groups: u32) -> libc::sockaddr_nl {
    let mut sock_addr = unsafe { std::mem::zeroed::<libc::sockaddr_nl>() };
    sock_addr.nl_family = libc::AF_NETLINK as u16;
    sock_addr.nl_pid = pid;
    sock_addr.nl_groups = groups;
    sock_addr
}

fn bind_nl_socket(socket: RawFd, address: libc::sockaddr_nl) -> Result<(), Errno> {
    // This cast looks strange in Rust but is pretty standard C for getting a
    // sockaddr pointer from sockaddr_nl. See "man 7 netlink" for example.
    let address_ptr = &address as *const libc::sockaddr_nl as *const libc::sockaddr;
    let address_len = std::mem::size_of::<libc::sockaddr_nl>() as libc::socklen_t;
    let status = unsafe { libc::bind(socket, address_ptr, address_len) };
    check_os_error(status)?;
    Ok(())
}

fn connect() -> Result<RawFd, Errno> {
    let socket = create_genl_socket()?;
    let address = create_nl_addr(0, 0);
    bind_nl_socket(socket, address)?;
    Ok(socket)
}

fn get_genl_family_msg(family_name: &str) -> Vec<u8> {
    let nl_header = libc::nlmsghdr {
        nlmsg_len: 0,
        nlmsg_type: libc::GENL_ID_CTRL as u16,
        nlmsg_flags: (libc::NLM_F_REQUEST | libc::NLM_F_ACK) as u16,
        nlmsg_seq: 0,
        nlmsg_pid: 0,
    };
    let genl_header = libc::genlmsghdr {
        cmd: libc::CTRL_CMD_GETFAMILY as u8,
        version: 2,
        reserved: 0,
    };
    let nlattr_header = libc::nlattr {
        nla_len: 0,
        nla_type: libc::CTRL_ATTR_FAMILY_NAME as u16,
    };
    let mut bytes: Vec<u8> = vec![];

    write_nlmsghdr(&mut bytes, nl_header);
    write_genlmsghdr(&mut bytes, genl_header);
    write_nlattr(&mut bytes, nlattr_header, family_name.as_bytes());

    bytes
}

// Pads the end of bytes if necessary. See libnl docs for a description of when this is necessary.
//
//    "Most netlink protocols enforce a strict alignment policy for all boundries.
//    The alignment value is defined by NLMSG_ALIGNTO and is fixed to 4 bytes.
//    Therefore all netlink message headers, begin of payload sections, protocol
//    specific headers, and attribute sections must start at an offset which is a
//    multiple of NLMSG_ALIGNTO."
//
// https://www.infradead.org/~tgr/libnl/doc/core.html#_message_format
fn write_padding_if_necessary(bytes: &mut Vec<u8>) {
    let new_len = nlmsg_align(bytes.len());
    bytes.resize(new_len, 0);
}

fn write_nlmsghdr(bytes: &mut Vec<u8>, nl_header: libc::nlmsghdr) {
    write_padding_if_necessary(bytes);
    bytes.extend_from_slice(&nl_header.nlmsg_len.to_ne_bytes()[..]);
    bytes.extend_from_slice(&nl_header.nlmsg_type.to_ne_bytes()[..]);
    bytes.extend_from_slice(&nl_header.nlmsg_flags.to_ne_bytes()[..]);
    bytes.extend_from_slice(&nl_header.nlmsg_seq.to_ne_bytes()[..]);
    bytes.extend_from_slice(&nl_header.nlmsg_pid.to_ne_bytes()[..]);
    Ok(())
}

fn write_genlmsghdr(bytes: &mut Vec<u8>, genl_header: libc::genlmsghdr) {
    write_padding_if_necessary(bytes);
    bytes.extend_from_slice(&genl_header.cmd.to_ne_bytes()[..])?;
    bytes.extend_from_slice(&genl_header.version.to_ne_bytes()[..])?;
    bytes.extend_from_slice(&genl_header.reserved.to_ne_bytes()[..])?;
    Ok(())
}

fn write_nlattr(
    bytes: &mut Vec<u8>,
    nlattr_header: libc::nlattr,
    nlattr_payload: &[u8],
) -> std::io::Result<()> {
    bytes.extend_from_slice(&nlattr_header.nla_len.to_ne_bytes()[..])?;
    bytes.extend_from_slice(&nlattr_header.nla_type.to_ne_bytes()[..])?;
    bytes.extend_from_slice(nlattr_payload)?;
    Ok(())
}
