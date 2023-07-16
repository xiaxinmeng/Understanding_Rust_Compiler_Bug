
#[proc_macro]
pub fn ip(input: TokenStream) -> TokenStream {
    // format and remove all spaces, or else IP parsing will fail
    let s = format!("{}", input).replace(" ", "");
    match ip_helper(&s) {
        Ok(stream) => stream,
        Err(_) => panic!("invalid IP address or subnet: {}", s),
    }
}

fn ip_helper(s: &str) -> Result<TokenStream, ()> {
    Ok(match s.parse() {
        Ok(IpAddr::V4(v4)) => {
            let octets = v4.octets();
            quote!(::ip::Ipv4Addr::new([#(#octets),*])).into()
        }
        Ok(IpAddr::V6(v6)) => {
            let octets = v6.octets();
            quote!(::ip::Ipv6Addr::new([#(#octets),*])).into()
        }
        Err(_) => {
            // try to parse as a subnet before returning error
            if !s.contains("/") {
                return Err(());
            }
            let parts: Vec<&str> = s.split("/").collect();
            if parts.len() != 2 {
                return Err(());
            }
            let ip: IpAddr = parts[0].parse().map_err(|_| ())?;
            let prefix: u8 = parts[1].parse().map_err(|_| ())?;
            match ip {
                IpAddr::V4(v4) => {
                    if prefix > 32 {
                        return Err(());
                    }
                    let octets = v4.octets();
                    quote!(::ip::Subnet::new(::ip::Ipv4Addr::new([#(#octets),*]), #prefix)).into()
                }
                IpAddr::V6(v6) => {
                    if prefix > 128 {
                        return Err(());
                    }
                    let octets = v6.octets();
                    quote!(::ip::Subnet::new(::ip::Ipv6Addr::new([#(#octets),*]), #prefix)).into()
                }
            }
        }
    })
}
