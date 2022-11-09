use std::error::Error;
use std::net::TcpStream;

use ssh2::Session;

pub mod config;
mod sftp;

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let tcp = TcpStream::connect(config.addr).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    sess.userauth_password(&config.username, &config.password)
        .unwrap();
    assert!(sess.authenticated(), "failed to authenticate session");

    let sftp_conn = sess.sftp().unwrap();

    sftp::list(sftp_conn, config.path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connection() {}

    #[test]
    fn build_config() {
        let args: Vec<String> = vec![
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
        ];

        let addr = "".to_string();
        let username = "".to_string();
        let password = "".to_string();
        let path = "".to_string();

        let res = config::Config::build(&args).unwrap();

        assert_eq!(
            config::Config {
                addr,
                username,
                password,
                path
            },
            res
        );
    }
}
