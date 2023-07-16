
// foo.rs

mod a {

    pub fn a() {
        error!("error from module a")
        warn!("warn from module a")
        info!("info from module a")
        debug!("debug from module a")
    }
}

mod b {

    pub fn b() {
       error!("error from module b")
       warn!("warn from module b")
       info!("info from module b")
       debug!("debug from module b")
    }
}

fn main() {
   a::a();
   b::b();
}
