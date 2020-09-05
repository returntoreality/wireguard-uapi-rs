use super::attr::ControllerAttribute;
use crate::netlink::attr::NetlinkAttributeRequest;
use crate::netlink::genl::socket::GenlSocket;
use crate::netlink::genl::GenericNetlinkHeader;
use crate::netlink::genl::GenericNetlinkRequest;
use crate::netlink::message::NetlinkPayloadRequest;
use crate::netlink::message::NetlinkPayloadResponse;

struct GetFamilyPayload<'a> {
    family_name: &'a str,
}

impl NetlinkPayloadRequest for GetFamilyPayload<'_> {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(self.family_name.as_bytes());
        // This is supposed to be a c-string, so push a null terminated byte.
        buf.push(0);
    }
}

#[derive(PartialEq)]
struct GetFamilyResponse {
    id: u16,
}

impl NetlinkPayloadResponse for GetFamilyResponse {
    type Error = std::convert::Infallible;

    fn deserialize(buf: &[u8]) -> Result<Self, Self::Error> {
        let mut bytes: [u8; 2] = Default::default();
        bytes.copy_from_slice(&buf);

        let id = u16::from_ne_bytes(bytes);
        Ok(GetFamilyResponse { id })
    }
}

pub fn get_family(sock: &GenlSocket, family_name: &str) -> nix::Result<u16> {
    let genl_request = GenericNetlinkRequest {
        header: GenericNetlinkHeader {
            cmd: libc::CTRL_CMD_GETFAMILY as u8,
            version: 0,
        },
        payload: NetlinkAttributeRequest {
            ty: libc::CTRL_ATTR_FAMILY_NAME as u16,
            payload: GetFamilyPayload { family_name },
        },
    };

    sock.send(genl_request)?;
    let resp = sock.recv::<Vec<ControllerAttribute>>()?;

    for attr in resp.payload.payload {
        match attr {
            ControllerAttribute::FamilyId(id) => return Ok(id),
            _ => {}
        }
    }

    Ok(0)
}

mod tests {
    use super::{super::NetlinkGenericController, GetFamilyPayload};
    use crate::netlink::{
        attr::{Nested, NetlinkAttributeRequest},
        genl::{
            ctrl::attr::{ControllerAttribute, ControllerAttributeMulticastGroup},
            socket::GenlSocket,
            GenericNetlinkHeader, GenericNetlinkRequest, GenericNetlinkResponse,
        },
        message::{NetlinkPayloadRequest, NetlinkPayloadResponse},
    };

    /// genl ctrl get name acpi_event
    #[test]
    fn request_serialization() {
        let actual = {
            let genl_request = GenericNetlinkRequest {
                header: GenericNetlinkHeader {
                    cmd: libc::CTRL_CMD_GETFAMILY as u8,
                    version: 0,
                },
                payload: NetlinkAttributeRequest {
                    ty: libc::CTRL_ATTR_FAMILY_NAME as u16,
                    payload: GetFamilyPayload {
                        family_name: "acpi_event",
                    },
                },
            };

            let mut buf = vec![];
            genl_request.serialize(&mut buf);
            buf
        };

        let expected = [
            0x03, 0x00, 0x00, 0x00, 0x0f, 0x00, 0x02, 0x00, 0x61, 0x63, 0x70, 0x69, 0x5f, 0x65,
            0x76, 0x65, 0x6e, 0x74, 0x00,
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_deserialization() -> anyhow::Result<()> {
        let recv_bytes = [
            0x01, 0x02, 0x00, 0x00, 0x0f, 0x00, 0x02, 0x00, 0x61, 0x63, 0x70, 0x69, 0x5f, 0x65,
            0x76, 0x65, 0x6e, 0x74, 0x00, 0x00, 0x06, 0x00, 0x01, 0x00, 0x18, 0x00, 0x00, 0x00,
            0x08, 0x00, 0x03, 0x00, 0x01, 0x00, 0x00, 0x00, 0x08, 0x00, 0x04, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x08, 0x00, 0x05, 0x00, 0x01, 0x00, 0x00, 0x00, 0x24, 0x00, 0x07, 0x00,
            0x20, 0x00, 0x01, 0x00, 0x08, 0x00, 0x02, 0x00, 0x03, 0x00, 0x00, 0x00, 0x12, 0x00,
            0x01, 0x00, 0x61, 0x63, 0x70, 0x69, 0x5f, 0x6d, 0x63, 0x5f, 0x67, 0x72, 0x6f, 0x75,
            0x70, 0x00, 0x00, 0x00,
        ];

        let actual: GenericNetlinkResponse<Vec<ControllerAttribute>> =
            NetlinkPayloadResponse::deserialize(&recv_bytes)?;

        let expected = GenericNetlinkResponse {
            header: GenericNetlinkHeader { cmd: 1, version: 2 },
            payload: vec![
                ControllerAttribute::FamilyName("acpi_event".to_string()),
                ControllerAttribute::FamilyId(24),
                ControllerAttribute::Version(1),
                ControllerAttribute::HeaderSize(0),
                ControllerAttribute::MaxAttr(1),
                ControllerAttribute::MulticastGroups(Nested(vec![
                    ControllerAttributeMulticastGroup::Id(3),
                    ControllerAttributeMulticastGroup::Name("acpi_mc_group".to_string()),
                ])),
            ],
        };

        assert_eq!(actual, expected);

        Ok(())
    }
}
