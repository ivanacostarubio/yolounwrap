use std::net::SocketAddrV4;

fn main() {

    println!("YOLO UNWRAPS!!");

    let socket_addr_string = match std::env::args().into_iter().skip(1).next() {
        Some(s) => s,
        None => String::from("127.0.0.1:90009")
    };
    
   if let Ok(socket_addr) = socket_addr_string.parse::<SocketAddrV4>(){
        println!("{:?}", socket_addr);
        println!("🍻");
        println!("OK");
   }else{
       panic!("Could not parse lnd host. Please use IP:PORT!");
   }
   
   

}

