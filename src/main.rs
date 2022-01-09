use std::env;
use std::io::{Error, ErrorKind};
use std::net::{IpAddr, ToSocketAddrs};
use std::str::FromStr;

/// Updates dynamic ip managed by google domains
///
/// # Arguments
///
/// * `host` - dynamic host name
/// * `user` - user credential
/// * `passord` - password
///
/// # Example
/// app_name dyn.example.com user passw

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 4, "Missing required parameters");
    
    let host = &args[1];
    let user = &args[2];
    let password = &args[3];
    let ip_addr = dns_lookup(&host);
    let ip_addr = match ip_addr {
        Ok(ip_addr) => ip_addr,
        Err(_) => panic!("unable to resolve host"),
    };
    let my_ip_addr = my_ip();
    let my_ip_addr = match my_ip_addr {
        Ok(my_ip_addr) => my_ip_addr,
        Err(_) => panic!("Unable to get current ip"),
    };

    if my_ip_addr == ip_addr {
        println!("No change in ip {:?}", ip_addr);
        return;
    }

    let result = match update_ip(&host, &user, &password, my_ip_addr) {
        Ok(bool) => bool,
        Err(_) => panic!("Unable to change current ip"),
    };
    if result {
        println!("ip updated succesfully {:?}", my_ip_addr)
    } else {
        println!("ip update failed")
    }
}

fn dns_lookup(domain: &str) -> Result<IpAddr, Error> {
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

fn my_ip() -> Result<IpAddr, reqwest::Error> {
    let resp = reqwest::blocking::get("https://api.ipify.org")?.text()?;
    Ok(IpAddr::from_str(&resp).unwrap())
}

fn update_ip(hostname: &str, user: &str, passwd: &str, ip: IpAddr) -> Result<bool, reqwest::Error> {
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
