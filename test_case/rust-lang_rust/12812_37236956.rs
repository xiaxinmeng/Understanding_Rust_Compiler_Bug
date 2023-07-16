 rust
if SSL_VERSION == 2 {
        match *self {
            Sslv2 => {return ffi::SSLv2_method();},
            _ => {}
        }
}

match *self {
        Sslv3 => {return ffi::SSLv3_method();},
        Tlsv1 => {return ffi::TLSv1_method();},
        Sslv23 => {return ffi::SSLv23_method();}
}
