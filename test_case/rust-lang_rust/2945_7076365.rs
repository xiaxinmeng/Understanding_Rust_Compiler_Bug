
import task::{local_data_get, local_data_set};

fn display_callback_tls_key(+_callback: @int) {
    // Empty.
}

fn display_callback() unsafe {
    assert local_data_get(display_callback_tls_key).get() == @0;
}

fn main() {}

