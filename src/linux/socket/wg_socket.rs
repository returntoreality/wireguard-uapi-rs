use crate::get;
use crate::linux::attr::WgDeviceAttribute;
use crate::linux::cmd::WgCmd;
use crate::linux::consts::{WG_GENL_NAME, WG_GENL_VERSION};
use crate::linux::err::{ConnectError, GetDeviceError, SetDeviceError};
use crate::linux::set;
use crate::linux::set::create_set_device_messages;
use crate::linux::socket::parse::*;
use crate::linux::socket::NlWgMsgType;
use crate::linux::DeviceInterface;
use libc::IFNAMSIZ;
use neli::attr::Attribute;
use neli::consts::nl::{NlmF, NlmFFlags, Nlmsg};
use neli::consts::socket::NlFamily;
use neli::err::NlError;
use neli::genl::Genlmsghdr;
use neli::nl::{NlPayload, Nlmsghdr};
use neli::genl::Nlattr;
use neli::socket::{NlSocket, NlSocketHandle};
use neli::types::{Buffer, GenlBuffer};

pub struct WgSocket {
    sock: NlSocketHandle,
    family_id: NlWgMsgType,
}

impl WgSocket {
    pub fn connect() -> Result<Self, ConnectError> {
        let socket = NlSocketHandle::new(NlFamily::Generic)?;
        let family_id = socket.resolve_genl_family(WG_GENL_NAME)?;

        socket.bind(None, &[]);

        Ok(Self {
            sock: socket,
            family_id,
        })
    }

    pub fn get_device(
        &mut self,
        interface: DeviceInterface,
    ) -> Result<get::Device, GetDeviceError> {
        let mut mem = Buffer::new();
        let attr = match interface {
            DeviceInterface::Name(name) => {
                Some(name.len())
                    .filter(|&len| 0 < len && len < IFNAMSIZ)
                    .ok_or(GetDeviceError::InvalidInterfaceName)?;
                mem.extend_from_slice(&mem.as_ref());
                Nlattr::new(false, true, WgDeviceAttribute::Ifname, mem.as_ref())?
            }
            DeviceInterface::Index(index) => {
                mem.extend_from_slice(&index.to_be_bytes());
                Nlattr::new(false, true, WgDeviceAttribute::Ifindex, mem.as_ref())?
            }
        };
        let genlhdr = {
            let cmd = WgCmd::GetDevice;
            let version = WG_GENL_VERSION;
            let attrs = GenlBuffer::from_iter([attr].into_iter());
            Genlmsghdr::new(cmd, version, attrs)
        };
        let nlhdr = {
            let size = None;
            let nl_type = self.family_id;
            let flags = NlmFFlags::new(&[NlmF::Request, NlmF::Ack, NlmF::Dump]);
            let seq = None;
            let pid = None;
            let payload = genlhdr;
            Nlmsghdr::new(size, nl_type, flags, seq, pid, NlPayload::Payload(payload))
        };

        self.sock.send(nlhdr)?;

        let mut iter = self
            .sock
            .iter::<Nlmsg, Genlmsghdr<WgCmd, WgDeviceAttribute>>(false);

        let mut device = None;
        while let Some(Ok(response)) = iter.next() {
            match response.nl_type {
                Nlmsg::Error => return Err(GetDeviceError::AccessError),
                Nlmsg::Done => break,
                _ => (),
            };

            let handle = response.get_payload()?.get_attr_handle();
            device = Some(match device {
                Some(device) => extend_device(device, handle)?,
                None => parse_device(handle)?,
            });
        }

        device.ok_or(GetDeviceError::AccessError)
    }

    /// This assumes that the device interface has already been created. Otherwise an error will
    /// be returned. You can create a new device interface with
    /// [`RouteSocket::add_device`](./struct.RouteSocket.html#add_device.v).
    ///
    /// The peers in this device won't be reachable at their allowed IPs until they're added to the
    /// newly created device interface through a Netlink Route message. This library doesn't have
    /// built-in way to do that right now. Here's how it would be done with the `ip` command:
    ///
    ///
    /// ```sh
    ///  sudo ip -4 route add 127.3.1.1/32 dev wgtest0
    /// ```
    pub fn set_device(&mut self, device: set::Device) -> Result<(), SetDeviceError> {
        for nl_message in create_set_device_messages(device, self.family_id)? {
            self.sock.send(nl_message)?;
            self.sock.recv()?;
        }

        Ok(())
    }
}
