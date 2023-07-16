
if ((f.0 == 1 && 1 != 0 && 1 != 0) ||
	(f.0 == 2 && 1 != 0 && 1 != 0)) {
	i32 a;

	if (f.0 == 1 && 1 != 0 && 1 != 0) // matched on first alternative
	{
		a = f.1;
	}
	else {
		if (f.0 == 2 && 1 != 0 && 1 != 0) // matched on second alternative {
			a = f.2;
		}
	}

	// do something with a here
}
