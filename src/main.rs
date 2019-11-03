extern crate paho_mqtt as mqtt;

use std::{process, thread};
use std::sync::{Arc, Mutex};
use std::thread::Thread;
use std::time::Duration;

use futures::{Future, Stream};
use futures::future::ok;
use futures::future::lazy;
use mqtt::PersistenceType;

mod config;

fn on_connect_success(cli: &mqtt::AsyncClient, _msgid: u16) {
    println!("Connection succeeded");
    // Subscribe to the desired topic(s).
    cli.subscribe("test-topic", 0);
}

fn on_connect_failure(cli: &mqtt::AsyncClient, _msgid: u16, rc: i32) {
    println!("Connection attempt failed with error code {}.\n", rc);
    thread::sleep(Duration::from_millis(2500));
    cli.reconnect_with_callbacks(on_connect_success, on_connect_failure);
}

fn main() {
    dotenv::dotenv().expect("Cannot read .env file");

    env_logger::init();

    let config = config::Config::parse_from_envs();
    println!("Config: {:?}", &config);

    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(&config.server_uri)
        .client_id(&config.client_id)
        .persistence(PersistenceType::None)
        .finalize();

    let mut client = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
        println!("Error creating the client: {:?}", e);
        process::exit(1);
    });

    let connection_opts = mqtt::ConnectOptionsBuilder::new()
        .mqtt_version(mqtt::MQTT_VERSION_3_1_1)
        .keep_alive_interval(Duration::from_secs(60))
        .clean_session(true)
        .automatic_reconnect(Duration::from_millis(500), Duration::from_secs(5))
        .finalize();

    let rx = client.get_stream(4096);

    let msg_stream = rx.for_each(|opt_msg| {
        if let Some(msg) = opt_msg {
            println!("w1 {}", msg);
        } else {
            println!("w1 Stream disruption");
        }
        Ok(())
    });

    let connect_future = client.connect(connection_opts)
        .and_then(|(server_uri, _, _)| {
            Ok(())
        })
        .map_err(|e| {
            println!("Error: {}", e);
            process::exit(2);
        });

    tokio::run(lazy (|| {
        tokio::spawn(connect_future);

        Ok(())
    }));

    let subscribe_future = client.subscribe("test-topic", 0)
        .and_then(|(_)| {
            Ok(())
        })
        .map_err(|e| {
            println!("Error: {}", e);
            process::exit(3);
        });

    tokio::run(lazy (|| {
        tokio::spawn(subscribe_future);

        Ok(())
    }));

    tokio::run(lazy (|| {
        tokio::spawn(msg_stream);

        Ok(())
    }));
}
