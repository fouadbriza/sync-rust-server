use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;

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
        let response = match req_result {
            Ok(0) => {
                break
            },
            Ok(request)=> {
                let request_string = String::from_utf8_lossy(&buffer[..request]);
                println!("request received:\n\n{}",request_string);
                handle_routing(&request_string)
            },
            Err(_) => {println!("error receiving the request");
            break
            }
        };
        //  let body = format!("Hello {} from sync Rust server", cpt);
         cpt += 1;
        
        // let res_message = format!(
        //     "HTTP/1.1 200 OK\r\nConnection: keep-alive\r\nContent-Length: {}\r\n\r\n{}",
        //     body.len(),
        //     body
        // );
        
        let res_result = stream.write_all(response.as_bytes());
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



fn handle_routing(request: &str) -> String {

    let (status,filename) = if request.starts_with("GET / HTTP/1.1") {
        ("HTTP 200 Ok","html/index.html")
    }else{
        ("HTTP 404 NOT FOUND","html/404.html")
    };
    let body = fs::read_to_string(filename).unwrap_or_else(|_| { 
        format!("<h1>500 Internal Error: Missing {} on disk</h1>", filename)
    });

    format!("{}\r\nConnection: keep-alive\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        status,
        body.len(),
        body)

}