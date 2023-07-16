rust
> // ... On 32-bit and 16-bit we need to add 
> // an extra guard for this in case we're running on a platform which can use 
> // all 4GB in user-space, e.g., PAE or x32.
> 