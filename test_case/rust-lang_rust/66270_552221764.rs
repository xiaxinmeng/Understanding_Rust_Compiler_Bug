rust 
let Bug { any_field: Empty {} } // correct if we define `any_field: Empty` while 
let Bug { foo: 0 } // produces `refutable pattern` any way
