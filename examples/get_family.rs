use wireguard_uapi::netlink::genl::ctrl::NetlinkGenericController;
use wireguard_uapi::netlink::genl::socket::GenlSocket;

fn main() -> anyhow::Result<()> {
    let genl_controller = GenlSocket::connect()?;
    let family_id = genl_controller.get_family("acpi_event")?;

    assert_eq!(family_id, 0x18);
    Ok(())
}
