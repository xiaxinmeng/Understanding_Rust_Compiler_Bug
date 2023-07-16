
error: unneeded long form for URL
  --> $DIR/url-improvements.rs:5:5
   |
LL | /// [http://bb.com]
   |     ^^^^^^^^^^^^^^^ help: use an automatic link instead: `<http://bb.com>`
   |
   = note: bare URLs are not automatically turned into clickable links
