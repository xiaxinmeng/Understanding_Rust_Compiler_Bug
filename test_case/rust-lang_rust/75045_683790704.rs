
Replacing.1 t106: i32,i32 = ARMISD::SUBC t79, Constant:i32<1>

With: t106: i32,i32 = ARMISD::SUBC t79, Constant:i32<1>
 and 1 other values

Legalizing: t79: i32,i32 = ARMISD::ADDE Constant:i32<0>, Constant:i32<0>, t78:1
Legal node: nothing to do

Combining: t79: i32,i32 = ARMISD::ADDE Constant:i32<0>, Constant:i32<0>, t78:1

Legalizing: t78: i32,i32 = ARMISD::ADDE undef:i32, undef:i32, t82:1
Legal node: nothing to do

Combining: t78: i32,i32 = ARMISD::ADDE undef:i32, undef:i32, t82:1

Legalizing: t106: i32,i32 = ARMISD::SUBC t79, Constant:i32<1>
Legal node: nothing to do

Combining: t106: i32,i32 = ARMISD::SUBC t79, Constant:i32<1>

Replacing.1 t106: i32,i32 = ARMISD::SUBC t79, Constant:i32<1>

With: t106: i32,i32 = ARMISD::SUBC t79, Constant:i32<1>
 and 1 other values
