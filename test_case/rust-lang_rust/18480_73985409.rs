
// From libstd/path.rs
 fn parse_single_component(comp: &[u8]) -> Option<Component> {
+    const DOT: &'static [u8] = b".";
+    const DOTDOT: &'static [u8] = b"..";
+    const EMPTY: &'static [u8] = b"";
     match comp {
-        b"." => Some(Component::CurDir),
-        b".." => Some(Component::ParentDir),
-        b"" => None,
+        DOT => Some(Component::CurDir),
+        DOTDOT => Some(Component::ParentDir),
+        EMPTY => None,
         _ => Some(Component::Normal(unsafe { u8_slice_as_os_str(comp) }))
     }
 }
