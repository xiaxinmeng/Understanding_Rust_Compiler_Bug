diff
1:  d32c80467db ! 1:  6c9a5805b0c rustdoc: Strip broken links in summaries
    @@ Commit message
      ## src/librustdoc/html/markdown.rs ##
     @@ src/librustdoc/html/markdown.rs: fn push(s: &mut String, text_length: &mut usize, text: &str) {
              *text_length += text.len();
    -     };
    +     }
      
     -    'outer: for event in Parser::new_ext(md, summary_opts()) {
     +    // NOTE: Make sure to update the same variable in `plain_text_summary`
2:  3f685a724e9 < -:  ----------- Deduplicate broken-link callback
