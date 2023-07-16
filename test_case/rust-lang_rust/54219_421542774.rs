bash
rg -USP 'match\s+.+\s+{\s+Ok\((\w+)\)\s+=>\s+\1.*\s+Err\((\w+)\)\s+=>\s+\2'

# This command searches this pattern:
# match xxx {
#   Ok(a) => a,
#   Err(b) => b
