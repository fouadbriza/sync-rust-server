use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("error binding to the port");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("connection established");
                handle_client(stream);

            },
            Err(err)=> {
                eprintln!("failed to connect: {}",err)},
        }
    }

    

}


fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0;1024];

    let read_result = stream.read(&mut buffer);

    match read_result {
        Ok(_) => println!("reading succefull"),
        Err(_) => println!("error reading request")
    }

    println!("server req : {}",String::from_utf8_lossy(&buffer));
    
    let res = "HTTP/1.1 200 OK\r\n\r\nHello from sync Rust server";
    let write_result = stream.write_all(res.as_bytes());
    match write_result {
        Ok(_) => println!("response sent succefully"),
        Err(_) => println!("error sending response")
    }
    let flush_result = stream.flush();

    match flush_result {
        Ok(_) => println!("flush succefully"),
        Err(_) => println!("error flushing"),
    }


}
