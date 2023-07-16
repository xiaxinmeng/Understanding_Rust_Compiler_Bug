rust
impl<'a> TryFrom<&'a [u8]> for IpAddr {
    type Error = AddrParseError;

    fn try_from(b: &'a [u8]) -> Result<IpAddr, Self::Error> {
        Parser::from_bytes(b).parse_with(|p| p.read_ip_addr())
    }
}
