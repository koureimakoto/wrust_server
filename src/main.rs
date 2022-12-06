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
    str::{self}
    , time::Instant,
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
#[path ="server_info_test.rs"]
mod server_info_tests;


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

struct ClientData {
    stream: TcpStream,
    http_request : HttpRequest,
    http_response: HttpResponse
}

trait ClientOperations {
    fn handle_the_request(stream: TcpStream) -> ClientData;
    fn send_response(&mut self);
}

impl ClientOperations for ClientData {
    fn handle_the_request(mut stream: TcpStream) -> ClientData {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<String> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        ClientData { 
            stream,
            http_request : http_request,
            http_response: HttpResponse::new()
        }
    }
    
    fn send_response(&mut self) {
        let  uri: &str = get_uri(&self.http_request[0]);
        let time_stamp = Instant::now();
        self.http_response.status   = String::from("HTTP/1.1 200 OK\r\n\r\n");
        self.http_response.contents = fs::read_to_string(uri).unwrap();

        // println!("{}")
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
