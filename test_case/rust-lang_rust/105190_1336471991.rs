plain
   |                       |
   |                       expected `&str`, found enum `Cow`
   |                       help: change the output type to match the trait: `&str`
   |
   = note: expected fn pointer `fn(&MoveToCopy) -> &str`
              found fn pointer `fn(&MoveToCopy) -> Cow<'_, str>`
For more information about this error, try `rustc --explain E0053`.
error: could not compile `rustc_mir_transform` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_mir_transform` due to previous error
