use std::{env, process};
use std::net::TcpStream;
use std::path::Path;

use ssh2::Session;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err|{
        println!("Problem while parsing command line arguments: {err}");
        process::exit(1);
    });

    println!("");
    println!("Addr: {}", config.addr);
    println!("Username: {}", config.username);
    println!("Password: {}", config.password);
    println!("Path: {}", config.path);

    let tcp = TcpStream::connect(config.addr).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    sess.userauth_password(&config.username, &config.password).unwrap();
    assert!(sess.authenticated());

    let sftp_conn = sess.sftp().unwrap();

    let patronage_path = Path::new(&config.path);

    let files = sftp_conn.readdir(patronage_path).unwrap();

    for i in files.iter() {
        println!("{:?}", i);
    }
}


struct Config {
    addr: String,
    username: String,
    password: String,
    path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() <= 2 {
            return Err("not enough arguments");
        }

        let addr = args[1].clone();
        let username = args[2].clone();
        let password = args[3].clone();
        let path= args[4].clone();
        
        Ok(Config {addr, username, password, path })
    }
}
