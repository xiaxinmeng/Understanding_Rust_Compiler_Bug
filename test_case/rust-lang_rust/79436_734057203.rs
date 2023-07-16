rust
// based on https://github.com/cuviper/autocfg/blob/1.0.1/src/version.rs#L7
// can be compare by using `>=`: https://github.com/cuviper/autocfg/blob/1.0.1/src/lib.rs#L218
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Version {
    major: usize,
    minor: usize,
    patch: usize,
}

fn parse_version(s: &str) -> Result<Version, Box<dyn std::error::Error + Send + Sync>> {
    let mut digits = s.splitn(3, '.'); // Maybe we can pass a string that is split into three from the beginning?
    let major = digits.next().ok_or("missing major version")?.parse()?; // using `.trim()` before `.parse()` is may be prefer? https://github.com/rust-lang/rust/issues/64796#issuecomment-622984135
    let minor = digits.next().ok_or("missing minor version")?.parse()?;
    let patch = digits.next().unwrap_or("0").parse()?;
    Ok(Version {
        major,
        minor,
        patch,
    })
}

