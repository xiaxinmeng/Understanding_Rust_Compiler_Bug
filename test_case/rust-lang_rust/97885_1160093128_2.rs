
error[E0282]: type annotations needed for `std::collections::HashMap<&core10_action_api::Fmid, V>`
   --> core10-validation/src/app_definition_validator.rs:249:44
    |
249 |                     .fold(HashMap::new(), |mut map, (caller, callee)| {
    |                                            ^^^^^^^ consider giving this closure parameter the explicit type `std::collections::HashMap<_, V>`, where the type parameter `V` is specified
    |
    = note: type must be known at this point

error[E0308]: mismatched types
   --> core10-validation/src/lib.rs:313:50
    |
313 |     let errors = AppDefinitionValidator::try_new(app_definition, module_definitions)
    |                                                  ^^^^^^^^^^^^^^ types differ in mutability
    |
    = note: expected mutable reference `&mut AppDefinition`
                       found reference `&AppDefinition`
