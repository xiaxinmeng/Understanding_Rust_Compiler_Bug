
error: `window_i` does not live long enough
   --> D:/eap/rust/0/gtk\src\auto\application.rs:102:9
    |
101 |             ffi::gtk_application_inhibit(self.to_glib_none().0, window_i.to_glib_none().0, flags.to_glib(), reason.to_glib_none().0)
    |                                                                 -------- borrow occurs here
102 |         }
    |         ^ `window_i` dropped here while still borrowed
103 |     }
    |     - borrowed value needs to live until here
