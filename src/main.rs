use std::net::{TcpListener, IpAddr, Ipv4Addr};

struct
ServerInfo {
    addr: IpAddr,
    port: String
}

impl  ServerInfo {
    pub fn
    new() -> ServerInfo{
        let loopback = "127.0.0.1";
        ServerInfo {
            addr: IpAddr::V4(loopback.parse::<Ipv4Addr>().unwrap()),
            port: String::from("6868")
        }
    }

    pub fn
    set_new_server_address_v4(&mut self, address: &str) {
        self.addr = IpAddr::V4(address.parse::<Ipv4Addr>().expect("Invalid format"))
    }

    pub fn
    set_new_server_port(&mut self, port: &str) {
        self.port = port.parse().expect("Apenas valores numéricos")
    }
}

#[cfg(test)]
mod server_info_tests {
    use super::*;

    type Mock<'a> = Vec<&'a str>;

    fn mock_v4<'a>() -> Mock<'a> {
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
    fn check_loopback_address() {
        let server_info: ServerInfo = ServerInfo::new();
        let loopback: IpAddr = IpAddr::V4(
            "127.0.0.1".parse::<Ipv4Addr>().unwrap()
        );
        assert_eq!(server_info.addr, loopback)
    }


    #[test]
    fn get_server_address_v4_test() {
        let mock: Mock = mock_v4();
        let mut server_info = ServerInfo::new();


        for ip in mock.iter() {
            server_info.set_new_server_address_v4(ip);
            assert_eq!(
                server_info.addr,
                IpAddr::V4(ip.parse::<Ipv4Addr>().unwrap()))
        }


    }
}

fn main() {
    


    let listener = TcpListener::bind("127.0.0.1:9999").unwrap();

}
