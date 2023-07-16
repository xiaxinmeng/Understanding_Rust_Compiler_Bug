 rust
use markdown::ExternalHtml;

// ...

// possibly on on L178
let external_files = match ExternalHtml::load(matches.opt_strs("html-in-header").as_slice(), 
                                              matches.opt_strs("html-before-content").as_slice(), 
                                              matches.opt_strs("html-after-content").as_slice()) {
     Some(ef) => ef,
     None => return 3,
};

// ...
        (false, true) => return markdown::render(input, output.unwrap_or(Path::new("doc")),
                                                 &matches, &external_html),

// ...

            match html::render::run(krate, output.unwrap_or(Path::new("doc")), &external_html) {

// ...
