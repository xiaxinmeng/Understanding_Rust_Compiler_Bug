rust
error[E0599]: no method named `find_fastqs` found for enum `fastq_set::filenames::FastqDef` in the current scope
   --> turing_pd/src/ligation_qc_process_reads.rs:294:36
    |
294 |             for fastq in fastq_def.find_fastqs().unwrap() {
    |                                    ^^^^^^^^^^^ method not found in `fastq_set::filenames::FastqDef`
    |
   ::: /mnt/bazelbuild/user/joey.arthur/cargo/git/checkouts/fastq_set-9504d5db1bfdb3fb/69af0bc/src/filenames/mod.rs:22:8
    |
22  |     fn find_fastqs(&self) -> Result<Vec<InputFastqs>, Error>;
    |        ----------- the method is available for `fastq_set::filenames::FastqDef` here
    |
    = help: items from traits can only be used if the trait is in scope
    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
            `use fastq_set::filenames::FindFastqs;`

error: unused import: `fastq_set::filenames::FindFastqs`
 --> turing_pd/src/ligation_qc_process_reads.rs:4:5
  |
4 | use fastq_set::filenames::FindFastqs;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  