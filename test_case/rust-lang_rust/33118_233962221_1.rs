
const C: u8 = 10; // C(0)
let C @ _ = 11; // C(1)
fn f() {
     match xxx {
         C => {} // C(2)
         _ => {}
     }
}
