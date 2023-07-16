rust
fn main() {
    let props = Props {
        field_1: 1,
        field_2: 1,
    };
    let props_2 = props.clone();

    let _ = |__yew_props: Props| {
        let _ = props_2; // No ICE when we remove the type annotation
    };
}
