
pub trait Config {
    fn save(&self, path: &Path) -> Result<(), ()> where Self: serde::Serialize { ... }
}
