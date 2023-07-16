
   88           drop(lock);
   89
   90           if old_handler != &default_handler {
-> 91               Box::from_raw(old_handler as *mut (Fn(&PanicInfo) + 'static + Sync + Send));
   92           }
   93       }
   94   }
(lldb) p default_handler
(void (*)(std::panicking::PanicInfo *)) $17 = &0x1000c4300
(lldb) expr -f hex -- **((size_t **) &old_handler + 0)
(size_t) $18 = 0x00000001000c4300 0x00000001000c4300
