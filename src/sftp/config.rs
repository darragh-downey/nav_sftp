use chrono::DateTime;
use chrono::offset::{Utc, FixedOffset};

#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    pub addr: String,
    pub username: String,
    pub password: String,
    pub path: String,
    pub days: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() <= 5 {
            return Err("not enough arguments");
        }

        let addr = args[1].clone();
        let username = args[2].clone();
        let password = args[3].clone();
        let path = args[4].clone();
        let days = args[5].clone();

        let dt = chrono::Utc::now();

        dt.and_then(days);

        Ok(Config {
            addr,
            username,
            password,
            path,
            days,
        })
    }
}
