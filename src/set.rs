use std::net::IpAddr;
use std::net::SocketAddr;

/// Documentation of each field comes from:
/// https://www.wireguard.com/xplatform/#configuration-protocol
#[derive(Debug, Default, PartialEq)]
pub struct Device<'a> {
    /// The value for this key should be a lowercase hex-encoded private key of
    /// the interface. The value may be an all zero string in the case of a set
    /// operation, in which case it indicates that the private key should be
    /// removed.
    pub private_key: Option<&'a [u8; 32]>,

    /// The value for this is a decimal-string integer corresponding to the
    /// listening port of the interface.
    pub listen_port: Option<u16>,

    /// The value for this is a decimal-string integer corresponding to the
    /// fwmark of the interface. The value may 0 in the case of a set operation,
    /// in which case it indicates that the fwmark should be removed.
    pub fwmark: Option<u32>,

    /// This key/value combo is only valid in a set operation, in which case it
    /// indicates that the subsequent peers (perhaps an empty list) should
    /// replace any existing peers, rather than append to the existing peer list.
    pub replace_peers: Option<bool>,

    pub peers: Vec<Peer<'a>>,
}

impl<'a> Device<'a> {
    pub fn private_key(mut self, private_key: &'a [u8; 32]) -> Self {
        self.private_key = Some(private_key);
        self
    }

    pub fn listen_port(mut self, listen_port: u16) -> Self {
        self.listen_port = Some(listen_port);
        self
    }

    pub fn fwmark(mut self, fwmark: u32) -> Self {
        self.fwmark = Some(fwmark);
        self
    }

    pub fn replace_peers(mut self, replace_peers: Option<bool>) -> Self {
        self.replace_peers = replace_peers;
        self
    }

    pub fn peers(mut self, peers: Vec<Peer<'a>>) -> Self {
        self.peers = peers;
        self
    }
}

/// Documentation of each field comes from:
/// https://www.wireguard.com/xplatform/#configuration-protocol
#[derive(Clone, Debug, PartialEq)]
pub struct Peer<'a> {
    /// The value for this key should be a lowercase hex-encoded public key of a
    /// new peer entry, which this command adds. The same public key value may
    /// not repeat during a single message.
    pub public_key: &'a [u8; 32],

    /// This key/value combo is only valid in a set operation, in which case it
    /// indicates that the previously added peer entry should be removed from the
    /// interface.
    pub remove: Option<bool>,

    /// This key/value combo is only valid in a set operation, in which case it
    /// causes the operation only occurs if the peer already exists as part of
    /// the interface.
    pub update_only: Option<bool>,

    /// The value for this key should be a lowercase hex-encoded preshared-key of
    /// the previously added peer entry. The value may be an all zero string in
    /// the case of a set operation, in which case it indicates that the
    /// preshared-key should be removed.
    pub preshared_key: Option<&'a [u8; 32]>,

    /// The value for this key is either IP:port for IPv4 or \[IP\]:port for
    /// IPv6, indicating the endpoint of the previously added peer entry.
    pub endpoint: Option<&'a SocketAddr>,

    /// The value for this is a decimal-string integer corresponding to the
    /// persistent keepalive interval of the previously added peer entry. The
    /// value 0 disables it.
    pub persistent_keepalive_interval: Option<u16>,

    /// This key/value combo is only valid in a set operation, in which case it
    /// indicates that the subsequent allowed IPs (perhaps an empty list) should
    /// replace any existing ones of the previously added peer entry, rather than
    /// append to the existing allowed IPs list.
    pub replace_allowed_ips: Option<bool>,

    /// The value for this is IP/cidr, indicating a new added allowed IP entry
    /// for the previously added peer entry. If an identical value already exists
    /// as part of a prior peer, the allowed IP entry will be removed from that
    /// peer and added to this peer.
    pub allowed_ips: Vec<AllowedIp<'a>>,
}

impl<'a> Peer<'a> {
    pub fn from_public_key(public_key: &'a [u8; 32]) -> Self {
        Self {
            public_key,
            remove: None,
            update_only: None,
            preshared_key: None,
            endpoint: None,
            persistent_keepalive_interval: None,
            replace_allowed_ips: None,
            allowed_ips: vec![],
        }
    }

    pub fn remove(mut self, remove: Option<bool>) -> Self {
        self.remove = remove;
        self
    }

    pub fn update_only(mut self, update_only: Option<bool>) -> Self {
        self.update_only = update_only;
        self
    }

    pub fn preshared_key(mut self, preshared_key: &'a [u8; 32]) -> Self {
        self.preshared_key = Some(preshared_key);
        self
    }

    pub fn endpoint(mut self, endpoint: &'a SocketAddr) -> Self {
        self.endpoint = Some(endpoint);
        self
    }

    pub fn persistent_keepalive_interval(mut self, persistent_keepalive_interval: u16) -> Self {
        self.persistent_keepalive_interval = Some(persistent_keepalive_interval);
        self
    }

    pub fn replace_allowed_ips(mut self, replace_allowed_ips: Option<bool>) -> Self {
        self.replace_allowed_ips = replace_allowed_ips;
        self
    }

    pub fn allowed_ips(mut self, allowed_ips: Vec<AllowedIp<'a>>) -> Self {
        self.allowed_ips = allowed_ips;
        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AllowedIp<'a> {
    pub ipaddr: &'a IpAddr,
    pub cidr_mask: u8,
}

impl<'a> AllowedIp<'a> {
    pub fn from_ipaddr(ipaddr: &'a IpAddr) -> Self {
        Self {
            ipaddr,
            cidr_mask: match ipaddr {
                IpAddr::V4(_) => 32,
                IpAddr::V6(_) => 128,
            },
        }
    }
}
