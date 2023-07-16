
   = help: `*const T` may include non-zero sized metadata, such as a slice length or `dyn Trait` vtable
   = help: cast through a pointer that satisfies `Pointee<Metadata=()>` first:
