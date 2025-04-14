use std::net::TcpListener;
use clap::Parser;

//config Module
pub mod config;

//handler Module
pub mod proxy;

fn main(){
    let server_info = config::configs::Config::parse();
    let listner = TcpListener::bind(&format!("{}:{}" , server_info.host , server_info.port )).unwrap();
    
    for stream in listner.incoming() {
        println!("The server is running on {0}:{1}", server_info.host , server_info.port);
        match stream {
            Ok(_stream) => {
                println!("The TCP stream is received sucessfully");
            }

            Err(_e) => {

            }
        }
    }
}
