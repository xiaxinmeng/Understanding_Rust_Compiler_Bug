
find . \( -name "*.rs" -o -name "*.md" \) -print0 | xargs -0 egrep "module[- ]level documentation"
