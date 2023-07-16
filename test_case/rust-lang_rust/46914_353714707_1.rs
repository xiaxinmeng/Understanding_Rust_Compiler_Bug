diff
-773      let buf = &mut (*db).ReparseTarget as *mut _;
+773        let buf = &mut (*db).ReparseTarget as *mut c::WCHAR;
