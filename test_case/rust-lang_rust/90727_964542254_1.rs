
       type parameter         type parameter
       |                      |
       v                      v
fn foo<T: Display + Debug>(t: T) { ... }
       ---^^^^^^^^^^^^^^^
       |                |
       |                trait bounds (there is also such as thing as lifetime bounds, but search index excludes them)
       generic arguments (which can include type parameters and lifetime parameters)


                           type parameters
                           |
                      vvvvvv
fn bar<T>() -> Result<T, i32> { ... }
