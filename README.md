# yahoo-weather-rs

The `yahoo-weather-rs` create downloads the actual weather data for a given location and transforms it into rust data structures.

---
## Usage
Add `yahoo-weather-rs` as a dependency in `Cargo.toml`:
```toml
[dependencies]
yahoo-weather = "0.2"
```

Use the `get_weather()` function to get the weather data.
```rust
extern crate yahoo_weather;

fn main() {
    // Request the data
    let weather = yahoo_weather("Berlin").unwrap();

    // print it to the console
    println!("Weather: {:?}", weather);
}
```

---
## License
Copyright © 2016 Robert Schütte

Distributed under the [MIT License](LICENSE).
