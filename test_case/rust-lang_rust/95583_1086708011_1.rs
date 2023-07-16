rust
trait AcpiHandler {
    type PhysicalAddr: Copy;
    ...
}
impl<H: AcpiHandler> AcpiTables<H> {
    pub unsafe fn from_rsdp(
        handler: H,
        rsdp_address: H::PhysicalAddr
    ) -> Result<AcpiTables<H>, AcpiError> {...}
}
