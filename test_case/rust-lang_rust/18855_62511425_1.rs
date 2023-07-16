
macro_rules! f64_milliseconds (
    ($d:expr) => {
        match $d.as_is() {
            (secs, nanos) => (secs as f64)*1000f64 + (nanos as f64)/1000_000f64
        }
    }
)

macro_rules! f64_microseconds (
    ($d:expr) => {
        match $d.as_is() {
            (secs, nanos) => (secs as f64)*1000_000f64 + (nanos as f64)/1000f64
        }
    }
)
