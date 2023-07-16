
// attributes with a trailing semi-colon apply to the 'containing item', in this case the crate. These are considered 'inner attributes'
#[link(name = "logging2",
       vers = "0.1",
       uuid = "91E62841-B9F3-4721-97C6-621BEEE2DA1B")]; // <- indicates an inner attribute

#[crate_type = "lib"];

// inner attributes must come before the items contents, e.g. use statements.
use std;

mod logging2;
