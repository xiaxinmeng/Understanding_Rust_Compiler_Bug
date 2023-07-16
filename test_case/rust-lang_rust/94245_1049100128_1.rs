
pub type SceneObjectLink = Arc<Mutex<SceneObject>>; // shareable link to scene object
pub type SceneObjectWeakLink = std::sync::Weak<Mutex<SceneObject>>; // weak shareable link to scene object
