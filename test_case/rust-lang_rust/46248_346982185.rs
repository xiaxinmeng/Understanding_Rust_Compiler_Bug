
[00:46:10] -  --> $DIR/issue-46209-private-enum-variant-reexport.rs:14:45

[00:46:10] +  --> $DIR/issue-46209-private-enum-variant-reexport.rs:14:32

[00:46:10]     |

[00:46:10]  14 |     pub use self::Lieutenant::{JuniorGrade, Full};

[00:46:10] -   |                                             ^^^^

[00:46:10] +   |                                ^^^^^^^^^^^

[00:46:10]  ...

[00:46:10]  25 |     enum Lieutenant {

[00:46:10]     |     - help: consider making the enum public: `pub `

[00:46:10]  

[00:46:10]  error: variant is private and cannot be reexported

[00:46:10] -  --> $DIR/issue-46209-private-enum-variant-reexport.rs:14:32

[00:46:10] +  --> $DIR/issue-46209-private-enum-variant-reexport.rs:14:45

[00:46:10]     |

[00:46:10]  14 |     pub use self::Lieutenant::{JuniorGrade, Full};

[00:46:10] -   |                                ^^^^^^^^^^^

[00:46:10] +   |                                             ^^^^
