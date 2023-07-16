rust

#[email(template = "contact-email")]
pub struct DomainContactEmail<'a> {
    pub domain: &'a str,
    pub sender: &'a ContactSender<'a>,
    pub message: &'a str,
    pub inbox_url: &'a str,
}
