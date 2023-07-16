rust
use std::time::Duration;

fn main() {
    let config_file = "0.00004205";
    let f = config_file.parse::<f64>().unwrap();
    let first_duration = Duration::from_secs_f64(f);
    let config_file = first_duration.as_secs_f64().to_string(); // 0.000042049
    let f = config_file.parse::<f64>().unwrap();
    let second_duration = Duration::from_secs_f64(f);

    assert_eq!(first_duration, second_duration); // desirable property
}
