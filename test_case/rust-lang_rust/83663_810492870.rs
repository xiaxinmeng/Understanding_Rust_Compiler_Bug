rust
#[derive(PartialEq)]
pub struct Blueprint {
    pub fuel_tank_size: u32,
    pub payload: u32,
    pub wheel_diameter: u32,
    pub wheel_width: u32,
    pub storage: u32,
}

#[no_mangle]
pub fn compare_two_arrays(a: &[Blueprint], b: &[Blueprint])->bool{
    a==b
}
