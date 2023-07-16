rust
//Working
let _:&FnOnce<(<() as Lt<'_>>::T,),Output=()> = &|_|{};
//Not working
let _:&FnOnce(<() as Lt<'_>>::T) = &|_|{};
