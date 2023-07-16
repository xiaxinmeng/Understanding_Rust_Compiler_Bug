 rust
use std::net_url;
use core::result;

fn main() {
        let urlstr = "test";

        // this works without error or warning
        let r: &result::Result<net_url::Url,~str> = &net_url::from_str(urlstr);
        result::get_ref( r );

        // this gives me:
        // error: illegal borrow: borrowed value does not live long enough
        // note: borrowed pointer must be valid for unknown scope: 59.  Please report a bug....
        result::get_ref( &net_url::from_str(urlstr) );
}
