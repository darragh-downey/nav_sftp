
use std::net::TcpStream;
use std::path::Path;
use std::error::Error;

use ssh2::Session;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
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

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    pub addr: String,
    pub username: String,
    password: String,
    pub path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connection() {

    }

    #[test]
    fn build_config() {
        let args: Vec<String> = vec!["".to_string(), "".to_string(), "".to_string(), "".to_string()];

        let addr = "".to_string();
        let username = "".to_string();
        let password = "".to_string();
        let path = "".to_string();

        let res = Config::build(&args).unwrap();

        assert_eq!(Config{addr, username, password, path}, res);
    }

}