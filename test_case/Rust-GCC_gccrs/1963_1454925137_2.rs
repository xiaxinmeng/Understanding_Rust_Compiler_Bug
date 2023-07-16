
class ADTType : public BaseType, public SubstitutionRef
{
public:
  enum ADTKind
  {
    STRUCT_STRUCT,
    TUPLE_STRUCT,
    UNION,
    ENUM
  };

