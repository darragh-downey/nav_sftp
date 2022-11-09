#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    pub addr: String,
    pub username: String,
    pub password: String,
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
        let path = args[4].clone();

        Ok(Config {
            addr,
            username,
            password,
            path,
        })
    }
}
