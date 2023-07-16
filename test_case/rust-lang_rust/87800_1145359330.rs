rust
let (host, port) = addr_str.split_once(':').unzip();
let host = host.unwrap_or(&addr_str);
let port = port.map(|p| p.parse::<u16>()).transpose()?;
