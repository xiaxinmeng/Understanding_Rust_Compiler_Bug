bash
#!/bin/bash

while read author; do
	read hash;
	[ -f hashes/$hash ] || \
	curl -f "https://api.github.com/repos/rust-lang/rfcs/commits/$hash" -o hashes/$hash || \
	echo "Error for commit $hash"
	[ -f hashes/$hash ] && cat hashes/$hash | jq -r -e '.author.login' | sed "s/^null$/null $hash/"
	while read hash; do
		[ -z "$hash" ] && break
	done
done

