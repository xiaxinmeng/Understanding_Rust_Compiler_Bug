
fn main(){
    let output_dir = from_str("/tmp/delete_me").unwrap();
    std::os::make_dir(&output_dir, 0b111_111_111);
    ::std::rt::io::support::PathLike::path_as_str(output_dir);
}
