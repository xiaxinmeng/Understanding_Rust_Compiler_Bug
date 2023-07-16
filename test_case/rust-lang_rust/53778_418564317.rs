
// Everything was expanded
macro m() {} // An Outer One, incorrect solution
{
	macro m() {} // An Outer One, incorrect solution
	{
		macro m() {} // An Outer One, incorrect solution
		{
			macro m() {} // The Inner One, correct solution
			
			m!(); // Or rather "trace" of it, since it was expanded too
		}
	}
}
