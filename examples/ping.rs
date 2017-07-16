extern crate homeassistant;

use std::env;

fn main() {
    let base_uri = env::var("HA_URL").unwrap(); // http://192.168.1.2:8123

    let client = homeassistant::Client::new(base_uri, None);

    let x = client.validate_api();
    println!("api is ok: {:?}", x);

    let x = client.ping().unwrap();
    println!("ping: {:?}", x);

    let x = client.get_config().unwrap();
    println!("config: {:?}", x);

    let x = client.get_discovery_info().unwrap();
    println!("discovery_info: {:?}", x);

    let x = client.get_event_listeners().unwrap();
    println!("events: {:?}", x);

    let x = client.get_services().unwrap();
    println!("services: {:?}", x);

    let x = client.get_states().unwrap();
    println!("states: {:?}", x);

    for y in x {
        let x = client.get_state(&y.entity_id).unwrap();
        println!("{:?}", x);
    }
}
