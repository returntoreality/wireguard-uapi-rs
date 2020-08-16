use std::collections::HashSet;
use wireguard_uapi::RouteSocket;

fn get_random_ifname() -> String {
    format!("wgtest{}", rand::random::<u16>())
}

#[test]
fn create_and_list_five_random_interfaces() -> anyhow::Result<()> {
    let mut route = RouteSocket::connect()?;

    let device_names_to_add: HashSet<String> = (1..=5).map(|_| get_random_ifname()).collect();
    for device_name in &device_names_to_add {
        route.add_device(device_name)?;
    }

    let received_device_names: HashSet<String> = route.list_device_names()?.into_iter().collect();
    assert!(device_names_to_add.is_subset(&received_device_names));

    for device_name in &device_names_to_add {
        route.del_device(device_name)?;
    }

    Ok(())
}
