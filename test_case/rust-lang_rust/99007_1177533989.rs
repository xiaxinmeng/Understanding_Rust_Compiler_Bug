plain
      Memory: 14 GB
      System Firmware Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMQqJQ33at/Z
      Provisioning UDID: 4203018E-580F-C1B5-9525-B745CECA79EB

hw.ncpu: 3
hw.byteorder: 1234
---
  |
3 | #![feature(unix_set_mark)]
  |            ^^^^^^^^^^^^^

error[E0599]: no method named `set_mark` found for struct `UnixDatagram` in the current scope
  |
  |
8 |     sock.set_mark(32)?;
  |          ^^^^^^^^ method not found in `UnixDatagram`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0599, E0635.
For more information about an error, try `rustc --explain E0599`.
---
  |
3 | #![feature(unix_set_mark)]
  |            ^^^^^^^^^^^^^

error[E0599]: no method named `set_mark` found for struct `UnixStream` in the current scope
  |
  |
8 |     sock.set_mark(32)?;
  |          ^^^^^^^^ method not found in `UnixStream`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0599, E0635.
For more information about an error, try `rustc --explain E0599`.
