 rust
let expression = {
    if param_id < args.len() {
        let test = &args.get(param_id).unwrap();
        test
    } else if let Some(ref e) = param.default_value {
        e
    } else {
        return Err(Error { kind: ErrorKind::ExpectedArgument(param.name.clone()), span: span})
    }
};
