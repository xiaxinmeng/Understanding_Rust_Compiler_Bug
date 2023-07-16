
pandoc --standalone --toc --number-sections  --from=markdown --to=latex src/doc/reference.md --output=doc/reference.tex
lualatex -interaction=batchmode -output-directory=doc doc/reference.tex
