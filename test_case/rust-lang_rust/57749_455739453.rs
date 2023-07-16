rust
let body = document.body().unwrap();
body.append_child(&{
    let button = document.create_element("button")?.dyn_into::<HtmlButtonElement>().unwrap();
    button.append_child(&document.create_text_node("Click me"))?;
    button
})?;
