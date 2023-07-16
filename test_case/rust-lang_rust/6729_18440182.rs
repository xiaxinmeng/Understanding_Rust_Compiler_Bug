
enum CStringBundle {
    allocation: Option<~[u8]>,
    c_str: *u8,
}

fn as_c_str(&self) -> CStringBundle {
    if I'm null terminated {
        CStringBundle { None, &self[0] }
    } else {
        let new_allocation = copy and null terminate me;
        CStringBundle { Some(new_allocation), &new_allocation[0] }
    } 
}
