rust
#[macro_use]
extern crate quicli;

// ...

use structopt::StructOpt;
use quicli::prelude::*;

// ...

#[derive(StructOpt)]
pub(crate) struct Opt {
    #[structopt(short = "d", long = "debug", help = "only use the local, println!() sink")]
    debug: bool,
    #[structopt(long = "verbose", short = "v", parse(from_occurrences))]
    verbose: u8,

    // for the SoundTouch source
    #[structopt(long = "soundtouch-host")]
    soundtouch_host: Option<String>,

    // for the last.fm source
    #[structopt(long = "lastfm-api-key")]
    lastfm_api_key: Option<String>,
    #[structopt(long = "lastfm-username")]
    lastfm_username: Option<String>,
}
