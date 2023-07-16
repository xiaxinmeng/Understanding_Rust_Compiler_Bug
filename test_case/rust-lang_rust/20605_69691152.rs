 rust

fn write_ppm16(
    out: &mut Writer, 
    dims: (u16, u16), 
    pixels: &mut iter::Iterator<Item = RGB16>,
    opt_conf: Option<PPMConfig>) -> IoResult<u32> {
// ...
for p in *pixels {
        try!(out.write_be_u16(p.r));
        try!(out.write_be_u16(p.g));
        try!(out.write_be_u16(p.b));

        bytes_written += 6;
    }
// ...
}
