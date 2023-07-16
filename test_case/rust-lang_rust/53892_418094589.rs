rust
/// Not timezone-aware
struct NaiveDateTime {
    pub year: …,
    pub month: …,
    pub day: …,
    pub hour: …,
    pub minute: …,
    pub second: …,
    pub nanosecond: …,
}

impl Display for NaiveDateTime {…}

impl SystemTime {
    pub fn to_utc(&self) -> NaiveDateTime {…}
    pub fn from_utc(dt: &NaiveDateTime) -> Self {…}
}
