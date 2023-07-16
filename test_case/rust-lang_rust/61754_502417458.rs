rust
      #[test]
      #[cfg_attr(target_os = "android", ignore)]
      #[cfg(unix)]
      fn uid_works() {
          use crate::os::unix::prelude::*;
  
          let mut p = Command::new("/bin/sh")
                              .arg("-c").arg("true")
                              .uid(unsafe { libc::getuid() })
                              .gid(unsafe { libc::getgid() })
                              .spawn().unwrap();                                                                                
          assert!(p.wait().unwrap().success());
      }
