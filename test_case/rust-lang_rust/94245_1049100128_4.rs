
pub enum ViewableData {
    Primitive(PrimitiveData),
    Sculpt(SculptData),
    MeshObject(MeshObjectData),
    Avatar(AvatarData),
    Animesh(AnimeshData),
    Unimplemented,
    Invalid, // if an error was detected or data absent.
}
