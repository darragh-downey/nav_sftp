use std::error::Error;
use std::net::TcpStream;
use std::path::Path;

use ssh2::{Session, Sftp};

pub mod config;

pub fn connect(config: &config::Config) -> Result<Sftp, Box<dyn Error>> {
    let tcp = TcpStream::connect(&config.addr).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    sess.userauth_password(&config.username, &config.password)
        .unwrap();
    assert!(sess.authenticated(), "failed to authenticate session");

    let sftp_conn = sess.sftp().unwrap();

    Ok(sftp_conn)
}

pub fn delete(_sftp_conn: &Sftp) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn list(sftp_conn: Sftp, path: String) -> Result<(), Box<dyn Error>> {
    let p = Path::new(&path);

    let files = sftp_conn.readdir(p).unwrap();

    for i in files.iter() {
        println!("{:?}", i);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connection() {}

    #[test]
    fn list_dir() {
        let args = vec![];
        let config = config::Config::build(&args).unwrap();

        let _sftp_conn = connect(&config).unwrap();
    }
}
