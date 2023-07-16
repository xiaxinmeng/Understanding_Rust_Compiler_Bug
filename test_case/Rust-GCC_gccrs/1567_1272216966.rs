cpp
// Inline-assembly specific options
enum InlineAsmOptions
{
  PURE = 1 << 0,
  NOMEM = 1 << 1,
  READONLY = 1 << 2,
  PRESERVES_FLAGS = 1 << 3,
  NORETURN = 1 << 4,
  NOSTACK = 1 << 5,
  ATT_SYNTAX = 1 << 6,
  RAW = 1 << 7,
  MAY_UNWIND = 1 << 8,
};

struct InlineAsmRegOrClass
{
public:
  /* constraint is a string because Rust inline assembly allows explicitly
     specifying a CPU-architectural-specific register */
  std::string constraint;
  /* is the constraint a class of registers (e.g. general registers)? or is it a
   * specific register (e.g. rax)? */
  bool is_constraint_class;
  Identifier name;
  Location locus;
};

struct InlineAsmOperand
{
public:
  enum RegisterType
  {
    IN,
    INOUT,
    OUT,
    LATEOUT,
    INLATEOUT,
    CONST,
    SYMBOL,
  };

  InlineAsmRegOrClass reg;
  bool is_anon_const;
  RegisterType type;
  // primary expression; in `inout` type, this is the in_expr
  std::unique_ptr<Expr> expr;
  // secondary expression, can be nullptr; this is the out_expr
  std::unique_ptr<Expr> other_expr;
  Location locus;
};

struct InlineAsmPlaceHolder
{
  size_t operand_idx;
  char modifier;
  Location locus;
};

struct InlineAsmTemplatePiece
{
  bool is_placeholder;
  union
  {
    std::string string;
    InlineAsmPlaceHolder placeholder;
  };
};
