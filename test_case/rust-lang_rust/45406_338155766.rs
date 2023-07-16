rust
if let IpAddr::V6(ipv6str) = en_due {
    if ipv6str == "dasdasdas" {
        println!("OK Ip tip V6");
    }
}

or

match en_due {
    IpAddr::V6(ipv6str) if ipv6str == "dasdasdas" => println!("OK Ip tip V6"),
    _ => {}
}
