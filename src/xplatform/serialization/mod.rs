use crate::xplatform::protocol::SetKey;
use crate::xplatform::set::AllowedIp;
use crate::xplatform::set::Device;
use crate::xplatform::set::Peer;
use std::fmt::Display;

impl Display for Device<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(private_key) = self.private_key {
            let private_key = hex::encode(private_key);
            writeln!(f, "{}={}", SetKey::PrivateKey, private_key)?;
        }

        if let Some(listen_port) = self.listen_port {
            writeln!(f, "{}={}", SetKey::ListenPort, listen_port)?;
        }

        if let Some(fwmark) = self.fwmark {
            writeln!(f, "{}={}", SetKey::Fwmark, fwmark)?;
        }

        if let Some(replace_peers) = self.replace_peers {
            writeln!(f, "{}={}", SetKey::ReplacePeers, replace_peers)?;
        }

        for peer in &self.peers {
            peer.fmt(f)?;
        }

        Ok(())
    }
}

impl Display for Peer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}={}", SetKey::PublicKey, hex::encode(self.public_key))?;

        if let Some(remove) = self.remove {
            writeln!(f, "{}={}", SetKey::Remove, remove)?;
        }

        if let Some(update_only) = self.update_only {
            writeln!(f, "{}={}", SetKey::UpdateOnly, update_only)?;
        }

        if let Some(preshared_key) = self.preshared_key {
            let preshared_key = hex::encode(preshared_key);
            writeln!(f, "{}={}", SetKey::PresharedKey, preshared_key)?;
        }

        if let Some(endpoint) = self.endpoint {
            writeln!(f, "{}={}", SetKey::Endpoint, endpoint)?;
        }

        if let Some(interval) = self.persistent_keepalive_interval {
            writeln!(f, "{}={}", SetKey::PersistentKeepaliveInterval, interval)?;
        }

        if let Some(replace_allowed_ips) = self.replace_allowed_ips {
            writeln!(f, "{}={}", SetKey::ReplaceAllowedIps, replace_allowed_ips)?;
        }

        for allowed_ip in &self.allowed_ips {
            allowed_ip.fmt(f)?;
        }

        Ok(())
    }
}

impl Display for AllowedIp<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}={}/{}",
            SetKey::AllowedIp,
            self.ipaddr,
            self.cidr_mask
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_website_example() {
        // Some of the lines have been re-arranged from the website example to
        // make testing easier.
        let expected = "\
            private_key=e84b5a6d2717c1003a13b431570353dbaca9146cf150c5f8575680feba52027a\n\
            listen_port=12912\n\
            fwmark=0\n\
            replace_peers=true\n\
            public_key=b85996fecc9c7f1fc6d2572a76eda11d59bcd20be8e543b15ce4bd85a8e75a33\n\
            preshared_key=188515093e952f5f22e865cef3012e72f8b5f0b598ac0309d5dacce3b70fcf52\n\
            endpoint=[abcd:23::33%2]:51820\n\
            replace_allowed_ips=true\n\
            allowed_ip=192.168.4.4/32\n\
            public_key=58402e695ba1772b1cc9309755f043251ea77fdcf10fbe63989ceb7e19321376\n\
            endpoint=182.122.22.19:3233\n\
            persistent_keepalive_interval=111\n\
            replace_allowed_ips=true\n\
            allowed_ip=192.168.4.6/32\n\
            public_key=662e14fd594556f522604703340351258903b64f35553763f19426ab2a515c58\n\
            endpoint=5.152.198.39:51820\n\
            replace_allowed_ips=true\n\
            allowed_ip=192.168.4.10/32\n\
            allowed_ip=192.168.4.11/32\n\
            public_key=e818b58db5274087fcc1be5dc728cf53d3b5726b4cef6b9bab8f8f8c2452c25c\n\
            remove=true\n";

        let peer1_test_ip = "192.168.4.4".parse().unwrap();
        let peer1_test_endpoint = "[abcd:23::33%2]:51820".parse().unwrap();
        let peer2_test_ip = "192.168.4.6".parse().unwrap();
        let peer2_test_endpoint = "182.122.22.19:3233".parse().unwrap();
        let peer3_test_ip1 = "192.168.4.10".parse().unwrap();
        let peer3_test_ip2 = "192.168.4.11".parse().unwrap();
        let peer3_test_endpoint = "5.152.198.39:51820".parse().unwrap();

        let set_request = Device {
            private_key: Some(&[
                0xe8, 0x4b, 0x5a, 0x6d, 0x27, 0x17, 0xc1, 0x00, 0x3a, 0x13, 0xb4, 0x31, 0x57, 0x03,
                0x53, 0xdb, 0xac, 0xa9, 0x14, 0x6c, 0xf1, 0x50, 0xc5, 0xf8, 0x57, 0x56, 0x80, 0xfe,
                0xba, 0x52, 0x02, 0x7a,
            ]),
            listen_port: Some(12912),
            fwmark: Some(0),
            replace_peers: Some(true),
            peers: vec![
                {
                    let mut peer = Peer::from_public_key(&[
                        0xb8, 0x59, 0x96, 0xfe, 0xcc, 0x9c, 0x7f, 0x1f, 0xc6, 0xd2, 0x57, 0x2a,
                        0x76, 0xed, 0xa1, 0x1d, 0x59, 0xbc, 0xd2, 0x0b, 0xe8, 0xe5, 0x43, 0xb1,
                        0x5c, 0xe4, 0xbd, 0x85, 0xa8, 0xe7, 0x5a, 0x33,
                    ]);
                    peer.preshared_key = Some(&[
                        0x18, 0x85, 0x15, 0x09, 0x3e, 0x95, 0x2f, 0x5f, 0x22, 0xe8, 0x65, 0xce,
                        0xf3, 0x01, 0x2e, 0x72, 0xf8, 0xb5, 0xf0, 0xb5, 0x98, 0xac, 0x03, 0x09,
                        0xd5, 0xda, 0xcc, 0xe3, 0xb7, 0x0f, 0xcf, 0x52,
                    ]);
                    peer.replace_allowed_ips = Some(true);
                    peer.allowed_ips.push(AllowedIp {
                        ipaddr: &peer1_test_ip,
                        cidr_mask: 32,
                    });
                    peer.endpoint = Some(&peer1_test_endpoint);
                    peer
                },
                {
                    let mut peer = Peer::from_public_key(&[
                        0x58, 0x40, 0x2e, 0x69, 0x5b, 0xa1, 0x77, 0x2b, 0x1c, 0xc9, 0x30, 0x97,
                        0x55, 0xf0, 0x43, 0x25, 0x1e, 0xa7, 0x7f, 0xdc, 0xf1, 0x0f, 0xbe, 0x63,
                        0x98, 0x9c, 0xeb, 0x7e, 0x19, 0x32, 0x13, 0x76,
                    ]);
                    peer.replace_allowed_ips = Some(true);
                    peer.allowed_ips.push(AllowedIp {
                        ipaddr: &peer2_test_ip,
                        cidr_mask: 32,
                    });
                    peer.persistent_keepalive_interval = Some(111);
                    peer.endpoint = Some(&peer2_test_endpoint);
                    peer
                },
                {
                    let mut peer = Peer::from_public_key(&[
                        0x66, 0x2e, 0x14, 0xfd, 0x59, 0x45, 0x56, 0xf5, 0x22, 0x60, 0x47, 0x03,
                        0x34, 0x03, 0x51, 0x25, 0x89, 0x03, 0xb6, 0x4f, 0x35, 0x55, 0x37, 0x63,
                        0xf1, 0x94, 0x26, 0xab, 0x2a, 0x51, 0x5c, 0x58,
                    ]);
                    peer.endpoint = Some(&peer3_test_endpoint);
                    peer.replace_allowed_ips = Some(true);
                    peer.allowed_ips.push(AllowedIp {
                        ipaddr: &peer3_test_ip1,
                        cidr_mask: 32,
                    });
                    peer.allowed_ips.push(AllowedIp {
                        ipaddr: &peer3_test_ip2,
                        cidr_mask: 32,
                    });
                    peer
                },
                {
                    let mut peer = Peer::from_public_key(&[
                        0xe8, 0x18, 0xb5, 0x8d, 0xb5, 0x27, 0x40, 0x87, 0xfc, 0xc1, 0xbe, 0x5d,
                        0xc7, 0x28, 0xcf, 0x53, 0xd3, 0xb5, 0x72, 0x6b, 0x4c, 0xef, 0x6b, 0x9b,
                        0xab, 0x8f, 0x8f, 0x8c, 0x24, 0x52, 0xc2, 0x5c,
                    ]);
                    peer.remove = Some(true);
                    peer
                },
            ],
        };
        let actual = format!("{}", set_request);

        assert_eq!(expected, actual);
    }

    #[test]
    fn serialize_update_only() {
        let expected = [
            "private_key=e84b5a6d2717c1003a13b431570353dbaca9146cf150c5f8575680feba52027a",
            "public_key=b85996fecc9c7f1fc6d2572a76eda11d59bcd20be8e543b15ce4bd85a8e75a33",
            "update_only=true",
            "endpoint=[abcd:23::33%2]:51820",
            "replace_allowed_ips=true",
            "allowed_ip=192.168.4.4/32",
            "",
        ]
        .join("\n");

        let test_ipv4 = "192.168.4.4".parse().unwrap();
        let test_endpoint = "[abcd:23::33%2]:51820".parse().unwrap();
        let set_request = Device {
            private_key: Some(&[
                0xe8, 0x4b, 0x5a, 0x6d, 0x27, 0x17, 0xc1, 0x00, 0x3a, 0x13, 0xb4, 0x31, 0x57, 0x03,
                0x53, 0xdb, 0xac, 0xa9, 0x14, 0x6c, 0xf1, 0x50, 0xc5, 0xf8, 0x57, 0x56, 0x80, 0xfe,
                0xba, 0x52, 0x02, 0x7a,
            ]),
            peers: vec![{
                let mut peer = Peer::from_public_key(&[
                    0xb8, 0x59, 0x96, 0xfe, 0xcc, 0x9c, 0x7f, 0x1f, 0xc6, 0xd2, 0x57, 0x2a, 0x76,
                    0xed, 0xa1, 0x1d, 0x59, 0xbc, 0xd2, 0x0b, 0xe8, 0xe5, 0x43, 0xb1, 0x5c, 0xe4,
                    0xbd, 0x85, 0xa8, 0xe7, 0x5a, 0x33,
                ]);
                peer.update_only = Some(true);
                peer.replace_allowed_ips = Some(true);
                peer.allowed_ips.push(AllowedIp {
                    ipaddr: &test_ipv4,
                    cidr_mask: 32,
                });
                peer.endpoint = Some(&test_endpoint);
                peer
            }],
            ..Default::default()
        };
        let actual = format!("{}", set_request);

        assert_eq!(expected, actual);
    }

    // Simple comparisons to make default, partial_eq, and debug derive code covered.
    #[test]
    fn cover_derives() {
        let device1 = Device::default();
        let device2 = Device::default();
        assert_eq!(device1, device2);
        format!("{:?}", device1);

        let peer1 = Peer::from_public_key(&[
            0xb8, 0x59, 0x96, 0xfe, 0xcc, 0x9c, 0x7f, 0x1f, 0xc6, 0xd2, 0x57, 0x2a, 0x76, 0xed,
            0xa1, 0x1d, 0x59, 0xbc, 0xd2, 0x0b, 0xe8, 0xe5, 0x43, 0xb1, 0x5c, 0xe4, 0xbd, 0x85,
            0xa8, 0xe7, 0x5a, 0x33,
        ]);
        let peer2 = Peer::from_public_key(&[
            0xb8, 0x59, 0x96, 0xfe, 0xcc, 0x9c, 0x7f, 0x1f, 0xc6, 0xd2, 0x57, 0x2a, 0x76, 0xed,
            0xa1, 0x1d, 0x59, 0xbc, 0xd2, 0x0b, 0xe8, 0xe5, 0x43, 0xb1, 0x5c, 0xe4, 0xbd, 0x85,
            0xa8, 0xe7, 0x5a, 0x33,
        ]);
        assert_eq!(peer1, peer2);
        format!("{:?}", peer1);

        let allowed_ip1 = AllowedIp {
            ipaddr: &"::1".parse().unwrap(),
            cidr_mask: 64,
        };
        let allowed_ip2 = AllowedIp {
            ipaddr: &"::1".parse().unwrap(),
            cidr_mask: 64,
        };
        assert_eq!(allowed_ip1, allowed_ip2);
        format!("{:?}", allowed_ip1);
    }
}
