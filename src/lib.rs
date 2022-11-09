use std::error::Error;

pub mod sftp;

use sftp::config::Config;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let sftp_conn = sftp::connect(config).unwrap();
    sftp::list(sftp_conn, config.path.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let days = "".to_string();

        let res = Config::build(&args).unwrap();

        assert_eq!(
            Config {
                addr,
                username,
                password,
                path,
                days,
            },
            res
        );
    }
}
