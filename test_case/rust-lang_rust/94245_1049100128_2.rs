
#[derive(Debug)]
pub struct SceneObject {
    pub self_weak_link: SceneObjectWeakLink, // weak link to self for others to use
    pub world_object_link: Box<dyn WorldObjectLink>,
    pub viewable_data: ViewableData, // prim, mesh, etc. info.
    pub render_faces: Vec<RenderFace>,          // the faces of the object. If filled in, mesh is good.
    pub loaded_asset: Option<SceneObjectAsset>, // asset used to generate this scene object
    pub desired_asset: Option<SceneObjectAsset>, // asset that needs to be loaded
    pub initial_loc: WorldLocation,             // initial location at which created, for debug use only
}
