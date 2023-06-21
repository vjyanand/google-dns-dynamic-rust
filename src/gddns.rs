use std::io::{Error, ErrorKind};
use std::net::{IpAddr, ToSocketAddrs};
use std::str::FromStr;

pub fn dns_lookup(domain: &str) -> Result<IpAddr, Error> {
    let domain_with_ip = format!("{}:443", domain);
    let addrs_iter = domain_with_ip.to_socket_addrs();
    let mut addrs_iter = match addrs_iter {
        Ok(addrs_iter) => addrs_iter,
        Err(error) => return Err(error),
    };
    let socket_addr = addrs_iter.next();

    let ip_addr = match socket_addr {
        Some(ip) => ip.ip(),
        None => return Err(Error::new(ErrorKind::Other, "oh no!")),
    };
    return Ok(ip_addr);
}

pub fn my_ip() -> Result<IpAddr, reqwest::Error> {
    let resp = reqwest::blocking::get("https://api.ipify.org")?.text()?;
    Ok(IpAddr::from_str(&resp).unwrap())
}

pub fn update_ip(hostname: &str, user: &str, passwd: &str, ip: IpAddr) -> Result<bool, reqwest::Error> {
    let url = format!(
        "https://{}:{}@domains.google.com/nic/update?hostname={}&myip={}",
        user, passwd, hostname, ip
    );
    let client = reqwest::blocking::Client::new();
    let resp = client.post(url).send().unwrap().text().unwrap();

    if !(resp.starts_with("good") || resp.starts_with("nochg")) {
        return Ok(false);
    }
    Ok(true)
}
