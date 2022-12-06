use super::*;

type IpV4List<'a> = Vec<&'a str>;

fn mock_v4<'a>() -> IpV4List<'a> {
    let ip_list = vec![
        "28.85.62.177"   ,
        "104.62.25.152"  ,
        "165.218.4.101"  ,
        "34.86.172.121"  ,
        "128.63.235.150" ,
        "89.133.215.245" ,
        "102.239.195.76" ,
        "214.184.209.84" ,
        "138.82.202.179" ,
        "147.187.126.157",
        ];

    ip_list
}

#[test]
fn check_default_loopback_address_v4() {
    let server_info: ServerInfo = ServerInfo::new();
    assert_eq!(server_info.addr.is_loopback(), true)
}

#[test]
fn check_default_port() {
    let server_info: ServerInfo = ServerInfo::new();
    assert_eq!(server_info.port, "6868")
}


#[test]
fn check_server_address_v4_format() {
    let ip_v4_list: IpV4List = mock_v4();
    let mut server_info = ServerInfo::new();


    for ip in ip_v4_list.iter() {
        server_info.set_new_server_address_v4(ip);
        assert_eq!(
            server_info.addr,
            IpAddr::V4(ip.parse::<Ipv4Addr>().unwrap()))
    }
}
