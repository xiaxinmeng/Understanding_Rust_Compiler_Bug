
#[derive (Copy, Clone, Debug, PartialEq)]
pub struct WorldLocation {
    region_origin: DVec2,                       // origin of region
    pos_within_region: Vec3,                    // position within region, Rend3 order
}
