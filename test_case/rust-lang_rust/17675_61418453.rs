
fn main()
{
    let x = 5i;
    let y = 10i;

    match cmp(x, y)
    {
        Less => println!("less"),
        Equal => println!("equal"),
        Greater => println!("greater")
    }
}
