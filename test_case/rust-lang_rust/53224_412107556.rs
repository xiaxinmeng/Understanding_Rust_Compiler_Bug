rust
// Gets a mutable reference to resource’s register bank
unsafe fn get_resource(x: &ResourceLock) -> &mut Resource {
    // THE_RESOURCE is linked by the linker into well known hardware operated range of memory.
    #[link_section="something_wellknown"]
    static mut THE_RESOURCE: Resource;
    take_resource_lock(x); // later released by the caller once they’re finished with the Resource
    &mut THE_RESOURCE
}
