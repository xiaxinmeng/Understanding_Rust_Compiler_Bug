
let res: result<filedesc,fail> = some_io_operation();
if res.is_failure() { ... handle failure; ... }
let filedesc = res.get();
