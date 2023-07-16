cpp
class MDNode : public Metadata {
//...
  const MDOperand &getOperand(unsigned I) const {
    assert(I < NumOperands && "Out of range");
    return op_begin()[I];
  }
