 C
enum A_discrim { B, D };
struct A {
  union { enum A_discrim discrim; int unnamed; } B;
  union { enum A_discrim discrim; float foo; } D;
};
