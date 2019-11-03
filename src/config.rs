use std::env;

#[derive(Debug)]
pub struct Config {
    pub server_uri: String,
    pub client_id: String,

}

impl Config {
    pub fn parse_from_envs() -> Config {
        let server_uri = env::var("MQTT_SERVER_URI").unwrap_or("ws://127.0.0.1:9001".to_string());
        let client_id = env::var("MQTT_CLIENT_ID").unwrap_or("test-client-1".to_string());

        Config { server_uri, client_id }
    }
}
