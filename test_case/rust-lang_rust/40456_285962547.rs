
find . -name "*.md" | xargs gsed -i 's/\[\([^ ]*\)()`\]/[\1`]/g'
find . -name "*.rs" | xargs gsed -i 's/\[\([^ ]*\)()`\]/[\1`]/g'
