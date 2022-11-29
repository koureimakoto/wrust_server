// REFACTOR THE ENTIRE CODE ------

use std::{
    net::{
        TcpListener,
        IpAddr,
        Ipv4Addr,
        TcpStream
    }, 
    fmt::Display,
    io::{
        BufReader,
        BufRead,
        Write,
        Error
    },
    fs,
    str::{self},
    mem::{
        self,
        ManuallyDrop
    },
};

pub struct
ServerInfo {
    addr: IpAddr,
    port: String
}

impl Display for ServerInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", &self.addr, &self.port)
    }
}

impl  ServerInfo {
    pub fn
    new() -> ServerInfo{
        let loopback = "127.0.0.1";
        let port: &str = "6868";
        ServerInfo {
            addr: IpAddr::V4(loopback.parse::<Ipv4Addr>().unwrap()),
            port: String::from(port)
        }
    }

    pub fn
    config(&self) -> String {
        format!("{}", self)
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
}

fn main() {
    let server_info: ServerInfo = ServerInfo::new();

    let listener = TcpListener::bind(
        server_info.config()
    ).unwrap();


    /*
    OLD
    */
    // for stream in listener.incoming() {
    //     let stream : TcpStream = stream.unwrap();
    //     let mut request: TcpStream = stream.try_clone().unwrap();
    //     let request = handle_the_request(&mut request);
    //     let uri = get_uri(&*request[0]);
    //     handle_the_response(stream, &uri).unwrap();
    //     drop(uri)
    // }

    /*
    NEW
    */
    for stream in listener.incoming() {

    }

}

type HttpRequest = Vec<String>;

struct HttpResponse {
    status: String,
    contents: String
}

impl HttpResponse {
    pub fn new() -> HttpResponse {
        HttpResponse { 
            status: String::new(),
            contents: String::new()
        }
    }
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, 
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            self.status,
            self.contents.len(),
            self.contents
        )
    }
}

struct ClientData {
    http_request : HttpRequest,
    http_response: HttpResponse
}

trait ClientOperations {
    fn handle_connection(stream: TcpStream) -> ClientData;
    fn send_response(&self);
}

impl ClientOperations for ClientData {
    fn handle_connection(mut stream: TcpStream) -> ClientData {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<String> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        ClientData { 
            http_request: http_request,
            http_response: HttpResponse::new()
        }
    }
    
    fn send_response(&self) {
        todo!()
    }
}

fn get_uri(line: &str) -> &str {
    let x: Vec<&str> = line.split(' ').collect();
    if x[1] == "/" {
        &"index.html"
    } else {
        x[1]
    }
}


fn handle_the_response(mut stream: TcpStream, uri: &str) -> std::io::Result<()> {
    let status_line  = "HTTP/1.1 200 OK\r\n\r\n";
    let contents = fs::read_to_string(uri).unwrap();
    let length = contents.len();
    
    let http_response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(http_response.as_bytes())
} 
