mermaid
flowchart LR

U[my user procedural macro] -->|linked against| R[rust interface]
R -->|linked against| C[libproc_macro cpp]
G[gccrs] -->|linked against| C
