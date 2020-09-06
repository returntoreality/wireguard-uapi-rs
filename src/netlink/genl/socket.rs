use super::{GenericNetlinkRequest, GenericNetlinkResponse};
use crate::netlink::{
    message::{
        NetlinkMessageHeader, NetlinkMessageRequest, NetlinkMessageResponse, NetlinkPayloadRequest,
        NetlinkPayloadResponse,
    },
    serialize,
};
use nix::sys::socket::{
    bind, socket, AddressFamily, MsgFlags, NetlinkAddr, SockAddr, SockFlag, SockProtocol, SockType,
};
use std::os::unix::io::RawFd;

pub struct GenlSocket {
    fd: RawFd,
}

impl GenlSocket {
    pub fn connect() -> nix::Result<GenlSocket> {
        // Remove this after nix adds NetlinkGeneric as a SockProtocol variant.
        let protocol: SockProtocol =
            unsafe { std::mem::transmute::<libc::c_int, SockProtocol>(libc::NETLINK_GENERIC) };

        let fd = socket(
            AddressFamily::Netlink,
            SockType::Raw,
            SockFlag::empty(),
            protocol,
        )?;

        let addr = SockAddr::Netlink(NetlinkAddr::new(0, 0));

        // bind(fd, &addr)?;
        // let addr = nix::sys::socket::getsockname(fd)?;

        Ok(Self { fd })
    }

    pub fn send<T: NetlinkPayloadRequest>(
        &self,
        genl_request: GenericNetlinkRequest<T>,
    ) -> nix::Result<()> {
        let message = NetlinkMessageRequest {
            header: NetlinkMessageHeader {
                ty: libc::GENL_ID_CTRL as u16,
                flags: (libc::NLM_F_REQUEST | libc::NLM_F_ACK) as u16,
                seq: 1,
                pid: 0,
            },
            payload: genl_request,
        };

        let message_bytes = serialize(&message);

        nix::sys::socket::send(self.fd, &message_bytes, MsgFlags::empty())?;
        Ok(())
    }

    pub fn recv<T: NetlinkPayloadResponse>(
        &self,
    ) -> nix::Result<NetlinkMessageResponse<GenericNetlinkResponse<T>>> {
        let mut resp_bytes = vec![0; 32768];
        nix::sys::socket::recv(self.fd, &mut resp_bytes, MsgFlags::empty())?;

        // TODO: Remove the expect.
        let resp_message =
            NetlinkMessageResponse::<GenericNetlinkResponse<T>>::deserialize(&resp_bytes)
                .expect("Error deserializing");
        Ok(resp_message)
    }
}
