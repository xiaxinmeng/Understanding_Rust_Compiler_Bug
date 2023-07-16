
use sxml::*;

#[test]
fn test_high_level_api()
{
    // sxml returns "not implemented" for everything...
    match sxml::from_str("<hmm/>")
    {
        Ok(*) => assert false,
        Err(mesg) => assert *mesg == ~"not implemented",
    }
}

// https://github.com/mozilla/rust/issues/3505
#[test]
fn test_low_level_api()
{
    match sxml::parse_str("<hmm/>")
    {
        Ok(*) => assert false,
        Err(mesg) => assert *mesg == ~"not implemented",
    }
}

#[test]
fn test_inner_modules()
{
    info!("hmm");
    match sxml::parsing::parse_str("<hmm/>")
    {
        Ok(*) => assert false,
        Err(mesg) => assert *mesg == ~"not implemented",
    }
}
