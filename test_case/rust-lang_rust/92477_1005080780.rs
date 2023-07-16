rust
use url::Url;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Adlist {
    pub(crate) source: Url,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(crate) format: AdlistFormat,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
pub enum AdlistFormat {
    Hosts,
    Domains,
    DnsMasq,
}
