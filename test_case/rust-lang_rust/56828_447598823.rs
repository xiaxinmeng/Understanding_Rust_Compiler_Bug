rust
for x in (Thing { x })
{
    println!("This is the loop body.");
}

for x in Thing { x }
{
    println!("This is a block expression.");
}
