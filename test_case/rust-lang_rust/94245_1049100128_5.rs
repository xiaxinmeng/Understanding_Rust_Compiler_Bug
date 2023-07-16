
pub struct RenderFace {
    pub renderer_link: RendererLink,
    pub self_weak_link: RenderFaceWeakIndex,
    pub object_handle: Option<ObjectHandle>, // objects are not shared.
    pub face_material: Option<RenderMaterial>,
    pub face_mesh: Option<RenderFaceMeshLink>,
    pub mesh_transform: Option<Mat4>, // position of mesh relative to rendering origin
                                      //  ***NEED DATA FOR TEXTURE ANIMATION***
}
