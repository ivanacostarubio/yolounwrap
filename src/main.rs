use std::net::SocketAddr;


fn main() {

    println!("YOLO UNWRAPS!!");

    let default = "127.0.0.1:10009";
    let socket_addr_string = std::env::args()
        .into_iter().skip(1).next()
        .unwrap_or(default.to_owned());
    let socket_addr: SocketAddr = socket_addr_string.parse().unwrap();
    let host = socket_addr.ip().to_string();
}
