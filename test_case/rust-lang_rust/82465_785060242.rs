sh
#!/bin/sh
rm -rf target-bisect*; cargo doc;grep '"impl-AsRef%3CGus%3E' ` find . -name struct.Gus.html`
