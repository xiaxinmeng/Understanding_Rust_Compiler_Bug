rust
pub fn sum(a: &Stats, b: &Stats, res: &mut Stats)
{
    res.x = a.x + b.x;
    res.y = a.y + b.y;
    res.z = a.z + b.z;
}
