
export output_format;
export output_styl;
export config;

#[doc = "The type of document to output"]
enum output_format {
    #[doc = "Markdown"]
    markdown,
    #[doc = "HTML, via markdown and pandoc"]
    pandoc_html
}

#[doc = "How to organize the output"]
enum output_style {
    #[doc = "All in a single document"]
    one_doc,
    #[doc = "Each module in its own document"]
    doc_per_mod
}

#[doc = "The configuration for this rustdoc session"]
type config = {
    output_dir: &str,
    output_format: output_format,
    output_style: output_style,
    pandoc_cmd: option<&str>
};
