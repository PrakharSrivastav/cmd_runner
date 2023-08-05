#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    #[allow(dead_code)]
    kind: IpAddrKind,

    #[allow(dead_code)]
    address: String,
}

#[derive(Debug)]
enum IpAddrString {
    V4(String),
    V6(String),
}

#[allow(dead_code)]
impl IpAddrString {
    fn print(&self) {
        log::info!("IpAddrString is {:?}",self)
    }
}

#[allow(dead_code)]
pub fn run() {
    let v4 = IpAddrKind::V4;
    let v6 = IpAddrKind::V6;

    log::info!("v4-{:?}, v6-{:?}",v4,v6);

    let addr_v4 = IpAddr { kind: IpAddrKind::V4, address: String::from("127.0.0.1") };
    let addr_v6 = IpAddr { kind: IpAddrKind::V6, address: String::from("::1") };
    log::info!("addr-v4-{:#?}, addr-v6-{:?}",addr_v4,addr_v6);

    // enum with String type
    let addr_home = IpAddrString::V4(String::from("127.0.01"));
    let addr_loopback = IpAddrString::V6(String::from("::1"));
    log::info!("addr-v4-{:?}, addr-v6-{:?}",addr_home,addr_loopback);

    addr_home.print();
    addr_loopback.print();
}