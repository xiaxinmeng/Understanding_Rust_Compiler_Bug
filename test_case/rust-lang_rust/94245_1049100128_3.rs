
/// Backlink to world objects, outside this library
/// Will mostly be eliminated soon. It's used by the JSON input system.
pub trait WorldObjectLink: Debug + Send {
    fn build_prim_mesh(
        &self,
        face_id: u8,
        lod: Lod,
        viewable_attrs: ViewableAttrs,
    ) -> Result<MeshCoords, Error>;
    //  Renderer fetches sculpt texture.
    fn build_sculpt_mesh(
        &self,
        face_id: u8,
        lod: Lod,
        viewable_attrs: ViewableAttrs,
        sculpt_image: RgbImage,
    ) -> Result<MeshCoords, Error>;
    fn build_mesh_mesh(
        &self,
        face_id: u8,
        lod: Lod,
        viewable_attrs: ViewableAttrs,
        mesh_info: Vec<MeshCoords>,
    ) -> Result<MeshCoords, Error>;
    //  Get position within region.  Mostly for debug messages.
    fn get_pos_in_region(&self) -> Vec3;
    /// Get transform to region
    fn get_transform_to_region(&self) -> Mat4;
    /// Test use only
    fn test_get_mesh_coords(&self, _face_id: u8) -> &MeshCoords {
        panic!("Unimplemented");
    }
    fn fetch_face_mesh(
        &self,
        scene: &RenderScene,
        face_id: usize,
        lod: Lod,
    ) -> Result<RenderFaceMeshLink, Error> {
        panic!("Unimplemented");
    }

    /// Update visible scene to match this link.
    fn update(
        &self,
        scene: &RenderScene,
        region: &SceneRegion,
        render_faces: &mut Vec<RenderFace>,
        lod: Lod,
        use_json_mesh: bool,
    ) -> Result<(), Error>;
}

