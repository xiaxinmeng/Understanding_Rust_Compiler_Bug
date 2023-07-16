 rust
fn bucket_name_from_path <'a>(path: &'a str) -> String {
    let parts: Vec<&str> = path.split_str("/").collect();
    return match parts.get(0).slice_from(0) {
        "s3.amazonaws.com" =>  parts.get(1).to_string(),
        _ => name_from_vhost_style(*parts.get(0))
    }
}
fn name_from_vhost_style <'a>(vhostname: &'a str) -> String {
    let hostname_parts: Vec<&str> = vhostname.split_str(".").collect::<Vec<&str>>();
    let bucket_parts = hostname_parts.slice(0, hostname_parts.len() - 2);
    return bucket_parts.connect("");
}
