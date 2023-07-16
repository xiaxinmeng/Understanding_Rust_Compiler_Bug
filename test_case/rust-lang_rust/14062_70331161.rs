 rust
extern crate "serialize" as rustc_serialize;

use rustc_serialize::Encoder;

#[derive(RustcEncodable)]
struct Attr;

struct JSTracer;

impl Encoder for JSTracer {
    type Error = ();

    fn emit_nil(&mut self) -> Result<(), ()> { Ok(()) }
    fn emit_uint(&mut self, _v: uint) -> Result<(), ()> { Ok(()) }
    fn emit_u64(&mut self, _v: u64) -> Result<(), ()> { Ok(()) }
    fn emit_u32(&mut self, __v: u32) -> Result<(), ()> { Ok(()) }
    fn emit_u16(&mut self, _v: u16) -> Result<(), ()> { Ok(()) }
    fn emit_u8(&mut self, _v: u8) -> Result<(), ()> { Ok(()) }
    fn emit_int(&mut self, _v: int) -> Result<(), ()> { Ok(()) }
    fn emit_i64(&mut self, _v: i64) -> Result<(), ()> { Ok(()) }
    fn emit_i32(&mut self, _v: i32) -> Result<(), ()> { Ok(()) }
    fn emit_i16(&mut self, _v: i16) -> Result<(), ()> { Ok(()) }
    fn emit_i8(&mut self, _v: i8) -> Result<(), ()> { Ok(()) }
    fn emit_bool(&mut self, _v: bool) -> Result<(), ()> { Ok(()) }
    fn emit_f64(&mut self, _v: f64) -> Result<(), ()> { Ok(()) }
    fn emit_f32(&mut self, _v: f32) -> Result<(), ()> { Ok(()) }
    fn emit_char(&mut self, _v: char) -> Result<(), ()> { Ok(()) }
    fn emit_str(&mut self, _v: &str) -> Result<(), ()> { Ok(()) }
    fn emit_enum<F>(&mut self, _name: &str, f: F) -> Result<(), ()> where F: FnOnce(&mut JSTracer) -> Result<(), ()> {
        f(self)
    }
    fn emit_enum_variant<F>(&mut self, _v_name: &str, _v_id: uint, _len: uint, f: F) -> Result<(), ()> where F: FnOnce(&mut JSTracer) -> Result<(), ()> {
        f(self)
    }
    fn emit_enum_variant_arg<F>(&mut self, _a_idx: uint, f: F) -> Result<(), ()> where F: FnOnce(&mut JSTracer) -> Result<(), ()> {
        f(self)
    }
    fn emit_enum_struct_variant<F>(&mut self, _v_name: &str, _v_id: uint, _len: uint, f: F) -> Result<(), ()> where F: FnOnce(&mut JSTracer) -> Result<(), ()> {
        f(self)
    }
    fn emit_enum_struct_variant_field<F>(&mut self, _f_name: &str, _f_idx: uint, f: F) -> Result<(), ()> where F: FnOnce(&mut JSTracer) -> Result<(), ()> {
        f(self)
    }
    fn emit_struct<F>(&mut self, _name: &str, _len: uint, f: F) -> Result<(), ()> where F: FnOnce(&mut JSTracer) -> Result<(), ()> {
        f(self)
    }
    fn emit_struct_field<F>(&mut self, _f_name: &str, _f_idx: uint, f: F) -> Result<(), ()> where F: FnOnce(&mut JSTracer) -> Result<(), ()> {
        f(self)
    }
    fn emit_tuple<F>(&mut self, _len: uint, f: F) -> Result<(), ()> where F: FnOnce(&mut JSTracer) -> Result<(), ()> {
        f(self)
    }
    fn emit_tuple_arg<F>(&mut self, _idx: uint, f: F) -> Result<(), ()> where F: FnOnce(&mut JSTracer) -> Result<(), ()> {
        f(self)
    }
    fn emit_tuple_struct<F>(&mut self, _name: &str, _len: uint, f: F) -> Result<(), ()> where F: FnOnce(&mut JSTracer) -> Result<(), ()> {
        f(self)
    }
    fn emit_tuple_struct_arg<F>(&mut self, _f_idx: uint, f: F) -> Result<(), ()> where F: FnOnce(&mut JSTracer) -> Result<(), ()> {
        f(self)
    }
    fn emit_option<F>(&mut self, f: F) -> Result<(), ()> where F: FnOnce(&mut JSTracer) -> Result<(), ()> {
        f(self)
    }
    fn emit_option_none(&mut self) -> Result<(), ()> { Ok(()) }
    fn emit_option_some<F>(&mut self, f: F) -> Result<(), ()> where F: FnOnce(&mut JSTracer) -> Result<(), ()> {
        f(self)
    }
    fn emit_seq<F>(&mut self, _len: uint, f: F) -> Result<(), ()> where F: FnOnce(&mut JSTracer) -> Result<(), ()> {
        f(self)
    }
    fn emit_seq_elt<F>(&mut self, _idx: uint, f: F) -> Result<(), ()> where F: FnOnce(&mut JSTracer) -> Result<(), ()> {
        f(self)
    }
    fn emit_map<F>(&mut self, _len: uint, f: F) -> Result<(), ()> where F: FnOnce(&mut JSTracer) -> Result<(), ()> {
        f(self)
    }
    fn emit_map_elt_key<F>(&mut self, _idx: uint, f: F) -> Result<(), ()> where F: FnOnce(&mut JSTracer) -> Result<(), ()> {
        f(self)
    }
    fn emit_map_elt_val<F>(&mut self, _idx: uint, f: F) -> Result<(), ()> where F: FnOnce(&mut JSTracer) -> Result<(), ()> {
        f(self)
    }
}

fn main() {
    let a = Attr;
    //let _  = a.encode(&mut JSTracer);
}
