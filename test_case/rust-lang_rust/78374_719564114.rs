
error[E0609]: no field `include_default_target` on type `std::sync::Arc<config::Config>`
   --> src/docbuilder/rustwide_builder.rs:325:50
    |
325 |                 } = metadata.targets(self.config.include_default_target);
    |                                                  ^^^^^^^^^^^^^^^^^^^^^^ unknown field

error: aborting due to previous error
