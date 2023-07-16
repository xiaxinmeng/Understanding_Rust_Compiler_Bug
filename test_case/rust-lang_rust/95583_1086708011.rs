rust
trait AcpiHandler {...}
impl<H: AcpiHandler> AcpiTables<H> {
    pub unsafe fn from_rsdp(
        handler: H,
        rsdp_address: usize
    ) -> Result<AcpiTables<H>, AcpiError> {...}
}
