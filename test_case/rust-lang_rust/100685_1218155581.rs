cpp
void IEEEFloat::initFromF80LongDoubleAPInt(const APInt &api) {
   uint64_t i1 = api.getRawData()[0];
   uint64_t i2 = api.getRawData()[1];
   uint64_t myexponent = (i2 & 0x7fff);
   uint64_t mysignificand = i1;
   uint8_t myintegerbit = mysignificand >> 63;
   ...
}
