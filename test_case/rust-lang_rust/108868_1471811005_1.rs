rust
impl<'a: 'email, 'email> email::BodyTemplates<'email> for DomainContactEmail<'a> {
    type Text = DomainContactEmailText<'a, 'email>;
    type Html = DomainContactEmailHtml<'a, 'email>;
}
#[derive(askama::Template)]
#[template(path = "contact-email.txt")]
pub struct DomainContactEmailText<'a, 'email>(&'email DomainContactEmail<'a>);

impl<'a: 'email, 'email> From<&'email DomainContactEmail<'a>>
    for DomainContactEmailText<'a, 'email>
{
    fn from(email: &'email DomainContactEmail<'a>) -> Self {
        Self(email)
    }
}
impl<'a: 'email, 'email> std::ops::Deref for DomainContactEmailText<'a, 'email> {
    type Target = &'email DomainContactEmail<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(askama::Template)]
#[template(path = "contact-email.html")]
pub struct DomainContactEmailHtml<'a, 'email>(&'email DomainContactEmail<'a>);

impl<'a: 'email, 'email> std::ops::Deref for DomainContactEmailHtml<'a, 'email> {
    type Target = &'email DomainContactEmail<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a: 'email, 'email> From<&'email DomainContactEmail<'a>>
    for DomainContactEmailHtml<'a, 'email>
{
    fn from(email: &'email DomainContactEmail<'a>) -> Self {
        Self(email)
    }
}
