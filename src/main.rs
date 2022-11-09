use ssh2::Session;
use std::net::TcpStream;
use std::path::Path;

fn main() {
    let tcp = TcpStream::connect("").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    sess.userauth_password("", "").unwrap();
    assert!(sess.authenticated());

    let sftp_conn = sess.sftp().unwrap();

    let patronage_path = Path::new("");

    let files = sftp_conn.readdir(patronage_path).unwrap();

    for i in files.iter() {
        println!("{:?}", i);
    }
}
