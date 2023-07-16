 text
// Before
|                 | Example       | Number of # pairs allowed | Available characters | Escapes                             | Equivalent to |
|-----------------|---------------|---------------------------|----------------------|-------------------------------------|---------------|
| Character       | 'H'           | N/A                       | All unicode          | \' & Byte escapes & Unicode escapes | N/A           |
| String          | "hello"       | N/A                       | All unicode          | \" & Byte escapes & Unicode escapes | N/A           |
| Raw             | r##"hello"##  | 0...                      | All unicode          | N/A                                 | N/A           |
| Byte            | b'H'          | N/A                       | All ASCII            | \' & Byte escapes                   | u8            |
| Byte string     | b"hello"      | N/A                       | All ASCII            | \" & Byte escapes                   | &'static u8   |
| Raw byte string | br##"hello"## | 0...                      | All ASCII            | N/A                                 | &'static u8   |

// After
|                 | Example       | # sets | Characters  | Escapes             |
|-----------------|---------------|--------|-------------|---------------------|
| Character       | 'H'           | N/A    | All Unicode | \' & Byte & Unicode |
| String          | "hello"       | N/A    | All Unicode | \" & Byte & Unicode |
| Raw             | r##"hello"##  | 0...   | All Unicode | N/A                 |
| Byte            | b'H'          | N/A    | All ASCII   | \' & Byte           |
| Byte string     | b"hello"      | N/A    | All ASCII   | \" & Byte           |
| Raw byte string | br##"hello"## | 0...   | All ASCII   | N/A                 |

~~~~~~~~~~~~~~~~Here is where the table is cutoff in the PDF approximately ^
