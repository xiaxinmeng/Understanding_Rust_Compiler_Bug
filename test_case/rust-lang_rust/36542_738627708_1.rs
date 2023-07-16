bash
#!/bin/bash
# convert the output of 'cargo build' to github-compatible html
(
	echo '<pre>'
	cargo --color=always build 2>&1 | aha --no-header
	echo '</pre>'
) \
| sed -E 's/<span style="[^"]*color:([^;"]+);"/<span color="\1"/g' \
| sed -E 's/ style="[^"]*"//g' \
| xclip -i -sel clipboard
