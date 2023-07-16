diff
-   unsafe { addr_of_mut!((*ptr)[..layout_size]) }
+   unsafe { addr_of_mut!((&mut *ptr)[..layout_size]) }
