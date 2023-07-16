rust
> struct HbaCmdTbl {
>     _prdt_entry: [HbaPrdtEntry; 65535],
> }
>
> struct HbaPrdtEntry {
>     _non_empty: u32
> }
>
> fn main() {
>     let cmdtbl_ptr = std::ptr::null_mut::<HbaCmdTbl>();
>     let _cmdtbl = unsafe { cmdtbl_ptr.read_volatile() };
> }
> 