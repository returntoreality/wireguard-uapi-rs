use std::iter::FromIterator;

use crate::linux::consts::WG_GENL_NAME;
use libc::{IFLA_INFO_KIND, IFLA_LINKINFO};
use neli::consts::nl::{NlmF, NlmFFlags};
use neli::consts::rtnl::{Arphrd, IffFlags, Ifla, Rtm};
use neli::err::SerError;
use neli::genl::Nlattr;
use neli::nl::{NlPayload, Nlmsghdr};
use neli::rtnl::Ifinfomsg;
use neli::rtnl::Rtattr;
use neli::types::{Buffer, RtBuffer};
use neli::attr::Attribute;

const RTATTR_HEADER_LEN: libc::c_ushort = 4;

pub enum WireGuardDeviceLinkOperation {
    Add,
    Delete,
}

fn create_rtattr(rta_type: Ifla, rta_payload: Vec<u8>) -> Rtattr<Ifla, Buffer> {
    Rtattr::new(None, rta_type, rta_payload).unwrap()
}

pub fn link_message(
    ifname: &str,
    link_operation: WireGuardDeviceLinkOperation,
) -> Result<Nlmsghdr<Rtm, Ifinfomsg>, SerError> {
    let ifname = create_rtattr(Ifla::Ifname, ifname.as_bytes().to_vec());

    let link = {
        let rta_type = Ifla::UnrecognizedConst(IFLA_LINKINFO);
        let payload = {
            // The Rtattr struct doesn't have a add_nested_attribute field like Nlattr. To work
            // around this, we can create a Nlattr and manually serialize it to a byte vector.
            let mut payload = Buffer::new();
            let rtattr =
                Nlattr::new::<Vec<u8>>(false, false, IFLA_INFO_KIND, WG_GENL_NAME.as_bytes().to_vec())?;
            payload.as_ref().to_vec()
        };
        create_rtattr(rta_type, payload)
    };

    let infomsg = {
        let ifi_family =
            neli::consts::rtnl::RtAddrFamily::UnrecognizedConst(libc::AF_UNSPEC as u8);
        // Arphrd::Netrom corresponds to 0. Not sure why 0 is necessary here but this is what the
        // embedded C library does.
        let ifi_type = Arphrd::Netrom;
        let ifi_index = 0;
        let ifi_flags = IffFlags::empty();
        let ifi_change = IffFlags::empty();
        let rtattrs = RtBuffer::from_iter([ifname, link].into_iter());
        Ifinfomsg::new(ifi_family, ifi_type, ifi_index, ifi_flags, ifi_change, rtattrs)
    };

    let nlmsg = {
        let len = None;
        let nl_type = match link_operation {
            WireGuardDeviceLinkOperation::Add => Rtm::Newlink,
            WireGuardDeviceLinkOperation::Delete => Rtm::Dellink,
        };
        let flags = match link_operation {
            WireGuardDeviceLinkOperation::Add => {
                NlmFFlags::new(&[NlmF::Request, NlmF::Ack, NlmF::Create, NlmF::Excl])
            }
            WireGuardDeviceLinkOperation::Delete =>NlmFFlags::new(&[NlmF::Request, NlmF::Ack]),
        };
        let seq = None;
        let pid = None;
        let payload = NlPayload::Payload(infomsg);
        Nlmsghdr::new(len, nl_type, flags, seq, pid, payload)
    };

    Ok(nlmsg)
}
