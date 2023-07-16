 rust
extern mod syntax;
extern mod extra;
use std::hashmap::HashSet;
use syntax::{parse, visit};

fn main() {
    let filename = match std::os::args() {
        [_, name] => name,
        _ => {
            println("Expected the file path");
            fail!()
        }
    };
    let path = Path(filename);

    let parsesess = parse::new_parse_sess(None);
    let mut crate = parse::parse_crate_from_file(&path, ~[], parsesess);
    crate = syntax::ext::expand::expand_crate(parsesess, ~[], crate);

    let files = @mut HashSet::new();
    let visitor = visit::mk_vt(@visit::Visitor {
        visit_mod: |m, sp, id, tup| {
            let fname = parsesess.cm.span_to_filename(sp);
            if "<core-macros>" != fname {
                (*files).insert(fname);
            }
            visit::visit_mod(m, sp, id, tup)
        },
        .. *visit::default_visitor()
    });

    visit::visit_crate(crate, ((), visitor));

    let mut v = ~[];
    do files.consume |s| { v.push(s); }
    extra::sort::tim_sort(v);

    println(v.connect(" "));
}
