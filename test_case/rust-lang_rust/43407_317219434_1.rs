
{
    let mut flag = false;
    let mut v = if let $p = $ei {
       $em
    } else {
        flag = true;
        unsafe { uninitialized() }
    };
    if flag {
        v = $ef;
    }
    v
}
