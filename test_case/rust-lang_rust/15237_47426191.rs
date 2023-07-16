 rust
pub struct ExternalHtml {
    pub in_header: String,
    pub before_content: String,
    pub after_content: String
}

impl ExternalHtml {
    pub fn load(in_header: &[String], before_content: &[String],
                after_content: &[String]) -> Option<ExternalHtml> {
        match (load_external_files(in_header),
               load_external_files(before_content),
               load_external_files(after_content)) {
            (Some(ih), Some(bc), Some(ac)) => {
                Some(ExternalHtml {
                    in_header: ih,
                    before_content: bc,
                    after_content: ac
                })
            }
             _ => None
        }
    }
}

...

pub fn render(..., external_html: &ExternalHtml) { ...
