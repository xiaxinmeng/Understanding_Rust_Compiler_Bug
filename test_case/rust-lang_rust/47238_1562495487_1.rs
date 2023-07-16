rs
// dumpty.rs

const DATA: &str = "Oh hello there!";

#[cfg(test)]
mod tests {
	use super::DATA;

    #[test]
    fn testing() {
        DATA.as_str();
    }
}
