rust
let metadata = Path::new(onedrive_file).metadata().expect("failed");
println!("{:?}", metadata.file_attributes()); // 1056

let metadata2 = Path::new(onedrive_dir).metadata().expect("failed");
println!("{:?}", metadata2.file_attributes()); // 1040
