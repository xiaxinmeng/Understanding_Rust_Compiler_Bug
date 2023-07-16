
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SceneObjectAsset {
    pub uuid_key: UuidKey                       // lod, UUID, but not planar projection mode, because that is per-face
}

pub type UuidKey = (u8, Uuid); // LOD/UUID pair
