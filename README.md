# homeassistant-rs [![Build Status](https://travis-ci.org/kpcyrd/homeassistant-rs.svg?branch=master)](https://travis-ci.org/kpcyrd/homeassistant-rs) [![Crates.io](https://img.shields.io/crates/v/homeassistant.svg)](https://crates.io/crates/homeassistant)

```toml
[dependencies]
homeassistant = "0.1"
```

## Usage

```rust,no_run
extern crate homeassistant;

fn main() {
    let client = homeassistant::Client::new("http://192.168.1.2:8123".to_owned(), None);

    for response in client.get_states().unwrap() {
        let x = client.get_state(&response.entity_id);
        println!("{:?}", x);
    }
}
```

See `examples/`, `src/` and <https://home-assistant.io/developers/rest_api/> for help.

You can also look at [kpcyrd/huesaverd](https://github.com/kpcyrd/huesaverd) for inspiration.

## License

MIT

