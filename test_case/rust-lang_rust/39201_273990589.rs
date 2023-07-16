
<ubsandroid> I don't understand how they wouldn't
<ubsandroid> even in plain ASCII...
<awygle> don't those take non-mutable references to self anyway, so they couldn't do the mutate-in-place thing?
<awygle> i guess that may not be a "why", exactly...
<Havvy> ubsandroid: Wouldn't do what?
<Havvy> awygle: Well, theoretically could have been &mut str.
<ubsandroid> Havvy: wouldn't be able to return anything but String
<ubsandroid> yeah but it's to_...
<ubsandroid> which is an &T -> U conversion
<Havvy> ubsandroid: Oh, right.
<Havvy> I missed that part of the convention.
<Havvy> Should at least document that the String could have more unicode characters than it started with.
<ubsandroid> yeah
<ubsandroid> sure
