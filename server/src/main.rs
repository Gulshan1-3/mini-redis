use std::{io::{Read, Write}, net::TcpListener,};
fn main() {
    let listener  =  TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming(){
        match stream {
            Ok(_stream)=> {
                println!("new Connection established");
                handle_connection(_stream);
            }
            Err(e)=> {

                println!("Problem establishing connection:{}",e);
                std::process::exit(0);
            }
        }
    }

}


fn handle_connection (mut _stream: std::net::TcpStream){
    let d = "+PONG\r\n";
    let mut buffer = [0;1024];
    loop {
     let bytes_read = _stream.read(&mut buffer).unwrap();
     if bytes_read == 0 {
        return;
     }

     _stream
     .write_all(&d.as_bytes())
     .expect("Failed to write to the stream");
    }
}
