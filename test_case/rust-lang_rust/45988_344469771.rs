
error[E0277]: the trait bound `plugins::PluginTransform + 'static: std::marker::Sync` is not satisfied  
   --> src/project.rs:397:30
    |
397 |         self.pods.par_iter().map(|pod| -> Result<()> {  
    |                              ^^^ trait `plugins::PluginTransform + 'static: std::marker::Sync` not satisfied
    |
    = note: `plugins::PluginTransform + 'static` cannot be shared between threads safely
    = note: required because it appears within the type `Box<plugins::PluginTransform + 'static>`
    = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<Box<plugins::PluginTransform + 'static>>`
    = note: required because it appears within the type `alloc::raw_vec::RawVec<Box<plugins::PluginTransform + 'static>>`
    = note: required because it appears within the type `std::vec::Vec<Box<plugins::PluginTransform + 'static>>`
    = note: required because it appears within the type `plugins::Manager`
    = note: required because it appears within the type `std::option::Option<plugins::Manager>`
    = note: required because it appears within the type `project::Project`
    = note: required because it appears within the type `&project::Project`
    = note: required because it appears within the type `&&project::Project`
    = note: required because it appears within the type `[closure@src/project.rs:397:34: 424:10 op:&plugins::Operation, self:&&project::Project, export_dir:&&std::path::Path]`
