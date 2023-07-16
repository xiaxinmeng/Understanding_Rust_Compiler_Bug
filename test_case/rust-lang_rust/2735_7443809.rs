
class defer {
    f: fn@();
    new(f: fn@()) {
        self.f = f;
    }   
    drop { self.f(); }
}

fn main() {
    let _ = do defer {
        #error["Second"];
    };  
    #error["First"];
}
