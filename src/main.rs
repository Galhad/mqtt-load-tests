mod config;

fn main() {
    dotenv::dotenv().expect("Cannot read .env file");

    let config  = config::Config::parse_from_envs();

    println!("Config: {:?}", &config);
}
