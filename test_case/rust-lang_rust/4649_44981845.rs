
for entry in try!(fs::readdir(some_path)).move_iter(){
    match &entry {
        p if /* p a borrowing reference here */ p.is_file() => {
             /* p owned, reference released here */ ...
        }, ...
    }
}
