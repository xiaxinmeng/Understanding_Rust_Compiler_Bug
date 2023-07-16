
pub trait Any {
     fn size(&self) -> uint;
     fn type_name(&self) -> &'static str;
}
