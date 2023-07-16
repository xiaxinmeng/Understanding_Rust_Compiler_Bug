rust
#[repr(C)]
#[non_exhaustive]
pub struct SystemTable {
    pub hdr: TableHeader,
    pub firmware_vendor: AtomicPtr<u16>,
    pub firmware_revision: u32,

    pub console_in_handle: AtomicPtr<c_void>,
    pub con_in: AtomicPtr<c_void>,
    pub console_out_handle: AtomicPtr<c_void>,
    pub con_out: AtomicPtr<c_void>,
    pub standard_error_handle: AtomicPtr<c_void>,
    pub std_err: AtomicPtr<c_void>,
    pub runtime_services: AtomicPtr<c_void>,
    pub boot_services: AtomicPtr<c_void>,

    pub number_of_table_entries: usize,
    pub configuration_table: AtomicPtr<c_void>,
}
