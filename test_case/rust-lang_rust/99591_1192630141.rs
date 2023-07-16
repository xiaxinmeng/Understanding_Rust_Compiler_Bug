rust
#![allow(unused)]
use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::rc::Rc;

fn main() {
    let mut rc = RenderContext {
        local_helpers: BTreeMap::new(),
    };
    foo(&mut rc);
}

#[derive(Clone)]
struct RenderContext<'rc> {
    local_helpers: BTreeMap<String, Rc<dyn HelperDef + 'rc>>,
}

pub trait HelperDef {}

fn foo(rc: &mut RenderContext<'_>) {
    let mut local_rc = rc.clone();
    let local_ctx: BTreeMap<String, String> = BTreeMap::new();
    render(&local_ctx, &mut local_rc);
}

fn render<'rc>(ctx: &'rc BTreeMap<String, String>, rc: &mut RenderContext<'rc>) {}
