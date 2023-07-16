
if match(val, m1) { b1 or b3 }  // m1 is stricter than m2, so no need to have b2 here
else if match(val, m2) { b2 or b3 }
else { b3 }
