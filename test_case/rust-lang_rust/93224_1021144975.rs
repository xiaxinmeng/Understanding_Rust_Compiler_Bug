cpp
TEST(APFloatTest, FMA_issue_93224) {
    uint64_t u1 = 0x000000000000002D;
    uint64_t u2 = 0xC2D6C16C166666DE;
    uint64_t u3 = 0x0000000000000055;

    APFloat f1(*((double*)&u1));
    APFloat f2(*((double*)&u2));
    APFloat f3(*((double*)&u3));
    f1.fusedMultiplyAdd(f2, f3, APFloat::rmNearestTiesToEven);
}
