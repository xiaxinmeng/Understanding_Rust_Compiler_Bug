 rust
macro_rules! handle_element(
    ($tag:expr, $string:expr, $type_id:expr, $ctor:ident, [$( $field:ident : $field_init:expr),*] ) => (
        if eq_slice($tag, $string) {
            let _element = ~$ctor {
                parent: Element::new($type_id, ($tag).to_str()),
                $(
                    $field: $field_init,
                )*
            };
            unsafe {
                return Node::as_abstract_node(_element);
            }
        }
    )
)

fn main() {
    handle_element!(tag, "ul", HTMLUListElementTypeId, HTMLUListElement, []);
    handle_element!(tag, "img", HTMLImageElementTypeId, HTMLImageElement, [image: None]);
}
