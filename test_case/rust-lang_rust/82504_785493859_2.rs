diff
        macro __helper__ ([$dol:tt] $exported_name:ident) {
            macro_rules! $exported_name {() => ()}
+           pub(crate) use $exported_name;
        }
-           pub(crate) use recurse;
