rust
pub struct Blueprint {
    pub fuel_tank_size: u32,
    pub payload: u32,
    pub wheel_diameter: u32,
    pub wheel_width: u32,
    pub storage: u32,
}
impl PartialEq for Blueprint{
   fn eq(&self, other: &Self)->bool{
      (self.fuel_tank_size == other.fuel_tank_size)
           && (self.payload == other.payload)
           && (self.wheel_diameter == other.wheel_diameter)
           && (self.wheel_width == other.wheel_width)
           && (self.storage == other.storage)
   }
}
