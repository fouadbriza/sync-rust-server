use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("error binding to the port");
    let mut thread_count = 0;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                
                println!("*****THREAD NUM: {}",thread_count);

                std::thread::spawn(move || {
                println!("connection established");
                handle_client_persistent(stream);
                });
                thread_count+=1;
                

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
        Ok(_) => println!("reading successful"),
        Err(_) => {println!("error reading request");
        return
        }
    }

    println!("server req : {}",String::from_utf8_lossy(&buffer));
    
    let res = "HTTP/1.1 200 OK\r\n\r\nHello from sync Rust server";
    let write_result = stream.write_all(res.as_bytes());
    match write_result {
        Ok(_) => println!("response sent successfully"),
        Err(_) => println!("error sending response")
    }
    let flush_result = stream.flush();

    match flush_result {
        Ok(_) => println!("flush successful"),
        Err(_) => println!("error flushing"),
    }


}


fn handle_client_persistent(mut stream: TcpStream) {
    
    let mut cpt =0;

    loop {
        let mut buffer = [0;9999];
        let req_result = stream.read(&mut buffer);
        match req_result {
            Ok(0) => {
                break
            },
            Ok(request)=> {
                println!("request received:\n\n{}",String::from_utf8_lossy(&buffer[..request]))
            },
            Err(_) => {println!("error receiving the request");
            break
            }
        }
        let body = format!("Hello {} from sync Rust server", cpt);
        cpt += 1;
        
        let res_message = format!(
            "HTTP/1.1 200 OK\r\nConnection: keep-alive\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        
        let res_result = stream.write_all(res_message.as_bytes());
        match res_result {
            Ok(_)=> {
                println!("response sent")
            },
            Err(_) => {println!("error sending the response");
            break
        }
        }
        let flush_result = stream.flush();
        match flush_result {
            Ok(_) =>{println!("flush successful")}
            Err(_) => {println!("error flushing");
            break
        }
    }   

    }

}