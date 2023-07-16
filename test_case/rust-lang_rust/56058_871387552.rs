rust
use serde::Deserialize;
use serde_json; 

struct Coord(f64, f64);

impl<'de> Deserialize<'de> for Coord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Inner {
            x: f64,
            y: f64,
        }
        
        let Inner { x, y } = Inner::deserialize(deserializer)?;
        Ok(Coord(x, y))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_content = r#"[{"x":1.0,"y":1.0},{"x":2.0,"y":17.0}]"#;
    let _: Vec<(f64, f64)> = serde_json::from_str(db_content)?;
    Ok(())
}
