
// rustc -g --test klass.rc
#[link(name = "klass", vers = "0.1", uuid = "04400014-FB9C-44DD-B5DD-A49BE5613BD8")];
#[crate_type = "lib"];

#[warn(no_non_implicitly_copyable_typarams, no_vecs_not_implicitly_copyable)];

use std;

mod types;
