
pub trait Config: serde::Serialize {
    fn save(&self, path: &Path) -> Result<(), ()> { ... }
}
