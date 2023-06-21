use googlednsdynamic::gddns::{dns_lookup, my_ip, update_ip};
use std::env;

/// Updates dynamic ip managed by google domains
///
/// # Arguments
///
/// * `host` - dynamic host name
/// * `user` - user credential
/// * `password` - password
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
