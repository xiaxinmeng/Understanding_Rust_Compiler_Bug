
---- ./Makefile --------
all: xml-client

.PHONY : xml-client
xml-client:
    cd sxml && rustc sxml.rc
    cd xml-client && rustc -L ../sxml --test client.rc && ./client

---- sxml/parsing.rs --------
use result::{Result, Err};

pub fn parse_str(_text: &str) -> Result<Xml, @~str>
{
    Err(@~"not implemented")
}

---- sxml/sxml.rc --------
#[link(name = "sxml", vers = "1.0", uuid = "333BE970-5A76-40CD-A101-3DD27CB469E5")];
#[crate_type = "lib"];

extern mod std;

pub mod parsing;
pub mod validation;

---- sxml/sxml.rs --------
use result::{Err, Ok, Result};
use std::map::{HashMap};

pub use parsing::{parse_str};
pub use validation::{validate_xml};

pub type Attributes = HashMap<@~str, @~str>;

pub enum Xml
{
    Content(@~str),
    Element(@~str, Attributes, @[@Xml]),
}

pub fn from_str(text: &str) -> Result<Xml, @~str>
{
    do parse_str(text).chain |xml|
    {
        match validate_xml(&xml)
        {
            @~""   => Ok(xml),
            errors => Err(errors),
        }
    }
}

---- sxml/validation.rs --------
pub fn validate_xml(_xml: &Xml) -> @~str
{
    @~""
}

---- xml-client/client.rc --------
#[link(name = "client", vers = "0.2", uuid = "D71AEFEB-650C-46E5-91C3-36E9406AEE8E")];

extern mod std;
extern mod sxml (name = "sxml", vers = "1.0");

mod client;

---- xml-client/client.rs --------
use sxml::*;                // compiler error with this present (see below)

//use sxml::{from_str};     // unit test compiles and runs with this instead
//use sxml::parsing;

use result::{Err, Ok, Result};

#[test]
fn test_inner_modules()
{
    info!("hmm");
    match parsing::parse_str("<hmm/>")
    {
        Ok(*) => assert false,
        Err(mesg) => assert *mesg == ~"not implemented",
    }
}

// With `use sxml::*;` get:
//<core-macros>:5:31: 5:41 error: unresolved name: core::info
//<core-macros>:5     #macro[[#info[f, ...], log(core::info, #fmt[f, ...])]];
//                                               ^~~~~~~~~~
//<core-macros>:5:10: 5:59 note: in expansion of #info
//client.rs:11:4: 11:17 note: expansion site
//error: aborting due to previous error
