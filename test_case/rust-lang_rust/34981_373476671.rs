rust
#[validate(length(equal = 10))]
#[validate(range(min = 1.1, max = 10.8))]
#[validate(schema(function = "validate_category", skip_on_field_errors = false)]
