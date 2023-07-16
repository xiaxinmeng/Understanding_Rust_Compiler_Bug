
use euca::diff;
use euca::dom::Dom;
use euca::vdom::DomIter;

#[test]
fn from_empty() {
    let new: Dom<(), (), &()> = Dom::elem("div");

    let n = new.dom_iter();
    let mut storage = vec![];
    let patch_set = diff::diff(std::iter::empty(), n, &mut storage);
}
