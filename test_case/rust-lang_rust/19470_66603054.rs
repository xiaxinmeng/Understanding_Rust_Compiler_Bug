
Rule C0:
a crate
T type
T has-no-free-parameters
---------------------------------
a ⊢ T complete

Rule C1:
a crate
T type
a ⊢ T local
---------------------------------
a ⊢ T complete

Rule T0:
a crate
Q unsubstituted-type
Q defined-in a
∀i. Pᵢ type
---------------------------------
a ⊢ Q〈Pᵢ〉 local

Rule T1:
a crate
Q unsubstituted-type
∀i. Pᵢ type
∀i. a ⊢ Pᵢ complete
∃i. a ⊢ Pᵢ local
---------------------------------
a ⊢ Q〈Pᵢ〉 local


Rule R0:
a crate
Tr unsubstituted-trait
∀i. Pᵢ type
S type
Tr defined-in a
---------------------------------
〈Tr〈Pᵢ〉 for S〉 local

Rule R1:
a crate
Tr unsubstituted-trait
∀i. Pᵢ type
S type
a ⊢ S local
---------------------------------
〈Tr〈Pᵢ〉 for S〉 local

Rule R2:
a crate
Tr unsubstituted-trait
∀i. Pᵢ type
S type
a ⊢ S complete
∀i. a ⊢ Pᵢ complete
∃i. a ⊢ Pᵢ local
---------------------------------
〈Tr〈Pᵢ〉 for S〉 local

