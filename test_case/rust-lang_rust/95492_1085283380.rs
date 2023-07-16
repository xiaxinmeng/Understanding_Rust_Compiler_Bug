rust
> // ptr is `&AtomicPtr`, mask is `usize`
> let v = *ptr;
> *ptr = v.map_addr(|addr| addr & mask):
> 