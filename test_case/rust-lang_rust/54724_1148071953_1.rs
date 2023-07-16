
use expr_proc_macro_def_site::{
    inner_using_outer_declarations_via_fn, inner_using_outer_declarations_via_temp, outer,
};

fn main() {
    outer!();
}
