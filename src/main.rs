use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}, fs::{self},
};
fn main() {
    let listener = 
    TcpListener::bind("127.0.0.1:7878").unwrap(); //bei 0.0.0.0 wird ans internet gehosted

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
        
    }
}
//does the ownership change here? yes it has moved!
fn handle_connection(mut stream: TcpStream){
    //does the ownership change with the mutable reference here? no it doesnt!
    let buf_reader = BufReader::new(&mut stream);
    /* 
    let http_request: Vec<_> = buf_reader
        .lines()
        //what do these vertical bars do?
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    */
    
    //what does the #do again? it formats it better!
    //println!("Request: {:#?}", http_request);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    
    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes()).unwrap();
}