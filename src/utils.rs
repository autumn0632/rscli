// pub mod network;

use ipnetwork::IpNetwork;

pub fn check_ip(ip: &str) -> Result<IpNetwork, ipnetwork::IpNetworkError> {
    ip.parse::<IpNetwork>()
}

pub fn check_ip_cidr(ip: &str) -> Result<IpNetwork, ipnetwork::IpNetworkError> {
    if !ip.contains('/') {
        return Err(ipnetwork::IpNetworkError::InvalidCidrFormat(
            "CIDR format is invalid".to_string(),
        ));
    }
    ip.parse::<IpNetwork>()
}
