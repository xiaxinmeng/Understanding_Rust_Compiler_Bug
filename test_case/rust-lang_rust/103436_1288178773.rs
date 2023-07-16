plain
   Compiling rand v0.7.3
   Compiling remove_dir_all v0.5.2
   Compiling tempfile v3.1.0
   Compiling tokei v10.1.2 (D:\a\rust\rust\build\ct\tokei)
error[E0791]: `Debug` can't be derived on this `#[repr(packed)]` struct that does not derive `Copy`
    |
    |
263 |           #[repr(C)] #[derive(Debug)] $(#[$attrs])*
    |
   ::: C:\Users\runneradmin\.cargo\registry\src\github.com-1ecc6299db9ec823\winapi-0.2.8\src\mmreg.rs:290:1
    |
    |
290 | / STRUCT!{#[repr(packed)] struct WAVEFORMATEX {
291 | |     wFormatTag: ::WORD,
292 | |     nChannels: ::WORD,
293 | |     nSamplesPerSec: ::DWORD,
297 | |     cbSize: ::WORD,
298 | | }}
    | |__- in this macro invocation
    |
    |
    = note: this error originates in the derive macro `Debug` which comes from the expansion of the macro `STRUCT` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0791]: `Debug` can't be derived on this `#[repr(packed)]` struct that does not derive `Copy`
    |
    |
263 |           #[repr(C)] #[derive(Debug)] $(#[$attrs])*
    |
   ::: C:\Users\runneradmin\.cargo\registry\src\github.com-1ecc6299db9ec823\winapi-0.2.8\src\mmreg.rs:299:1
    |
    |
299 | / STRUCT!{#[repr(packed)] struct WAVEFORMATEXTENSIBLE {
300 | |     Format: ::WAVEFORMATEX,
301 | |     Samples: ::WORD,
302 | |     dwChannelMask: ::DWORD,
303 | |     SubFormat: ::GUID,
304 | | }}
    |
    = note: this error originates in the derive macro `Debug` which comes from the expansion of the macro `STRUCT` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0791]: `Debug` can't be derived on this `#[repr(packed)]` struct that does not derive `Copy`
    |
    |
263 |           #[repr(C)] #[derive(Debug)] $(#[$attrs])*
    |
    |
   ::: C:\Users\runneradmin\.cargo\registry\src\github.com-1ecc6299db9ec823\winapi-0.2.8\src\usbspec.rs:26:1
    |
26  | / STRUCT!{#[repr(packed)] struct USB_CONFIGURATION_DESCRIPTOR {
27  | |     bLength: ::UCHAR,
28  | |     bDescriptorType: ::UCHAR,
29  | |     wTotalLength: ::USHORT,
...   |
34  | |     MaxPower: ::UCHAR,
35  | | }}
    |
    = note: this error originates in the derive macro `Debug` which comes from the expansion of the macro `STRUCT` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0791]: `Debug` can't be derived on this `#[repr(packed)]` struct that does not derive `Copy`
    |
    |
263 |           #[repr(C)] #[derive(Debug)] $(#[$attrs])*
    |
   ::: C:\Users\runneradmin\.cargo\registry\src\github.com-1ecc6299db9ec823\winapi-0.2.8\src\winusb.rs:8:1
    |
    |
8   | / STRUCT!{#[repr(packed)] struct WINUSB_SETUP_PACKET {
9   | |     RequestType: ::UCHAR,
10  | |     Request: ::UCHAR,
11  | |     Value: ::USHORT,
12  | |     Index: ::USHORT,
13  | |     Length: ::USHORT,
14  | | }}
    |
    = note: this error originates in the derive macro `Debug` which comes from the expansion of the macro `STRUCT` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0791`.
