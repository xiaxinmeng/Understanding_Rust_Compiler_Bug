
impl TimeSpan for i64 {
    fn num_seconds(x: &Duration) -> i64 {
        x.num_seconds()
    }
}

impl TimeSpan for f64 {
    fn num_seconds(x: &Duration) -> f64 {
        (x.num_seconds() as f64) + (x.nanos as f64)/(NANOS_PER_SEC as f64)
    }
}
