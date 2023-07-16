rust
// This program compiles but asserts when std::any::Any is in scope,
// but compiles and runs successfully when it is not in scope.
use std::any::Any;

use std::any::TypeId;

trait Resource {
    fn type_id(&self) -> TypeId;
}

impl Resource for () {
    fn type_id(&self) -> TypeId { TypeId::of::<()>() }
}

fn main() {
    let tid_orig = TypeId::of::<()>();    
    
    let ref_res: &dyn Resource = &();
    let tid_from_ref = ref_res.type_id();
    
    let box_res: Box<dyn Resource> = Box::new(());
    let tid_from_box = box_res.type_id();
    
    dbg!(tid_orig, tid_from_ref, tid_from_box);
    
    assert_eq!(tid_orig, tid_from_ref); // passes
    assert_eq!(tid_orig, tid_from_box); // fails
}

