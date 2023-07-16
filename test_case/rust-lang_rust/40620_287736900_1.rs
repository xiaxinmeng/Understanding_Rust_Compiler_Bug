
#[cfg(windows)]
mod not_a_real_file; //[windows]~ ERROR file not found for module `not_a_real_file`
//[windows]~^ HELP name the file either not_a_real_file.rs or not_a_real_file\mod.rs inside the directory

#[cfg(not(windows))]
mod not_a_real_file; //[not(windows)]~ ERROR file not found for module `not_a_real_file`
//[not(windows)]~^ HELP name the file either not_a_real_file.rs or not_a_real_file/mod.rs inside the directory


fn main() {
    assert_eq!(mod_file_aux::bar(), 10);
}
