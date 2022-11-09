use std::error::Error;
use std::path::Path;

use ssh2::Sftp;

pub fn delete(sftp_conn: &Sftp) -> Result<(), Box<dyn Error>> {
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
