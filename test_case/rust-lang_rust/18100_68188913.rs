 rust
use std::io;


fn stream_file(input_file   : &Path, 
                output_file  : &Path) -> io::IoResult<()> {

    let  fin  = io::File::open(input_file);
    let  fout = io::File::create(output_file);

    let mut r = io::BufferedReader::new(fin);
    let mut w = io::BufferedWriter::new(fout);

    let mut buf               = Vec::from_elem(128*1024, 0u8);
    let mut stream_ended      = false;

    // comment from here
    let n_read      = try!( r.read(buf.slice_to_mut(128) ));
    println!("Read {} byte.", n_read);

    try!(w.write(buf.slice_to(128)));
    // to here

    while !stream_ended {
        let n_read   = try!( r.read(buf.as_mut_slice())); 
        println!("Read {} byte, buffer is {} byte.", n_read, buf.len());

        if n_read < buf.len() {
            // probably stream end
            buf.truncate(n_read); // file ends with header
            println!("Short read of {0} byte, stream has probably ended. Buffer len = {1} ", n_read, buf.len());
            stream_ended = true;
        } 

        try!(w.write(buf.as_slice() ));
    }
    Ok( () )
}



fn main() {
    stream_file(&Path::new("/boot/vmlinuz-linux"), &Path::new("Test") )
        .ok().expect("Failed to read!");
}
