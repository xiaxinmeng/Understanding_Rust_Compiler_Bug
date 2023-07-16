
cratesfyi=> select name, version, releases.id from builds inner join releases on releases.id = builds.rid inner join crates on crates.id = releases.crate_id where output like '%error[E0282]: type annotations needed%';
        name         | version |   id   
---------------------+---------+--------
 small-logger        | 0.2.1   |  17558
 diesel_infer_schema | 0.13.0  |  49451
 diesel_infer_schema | 0.13.0  |  49451
 diesel_infer_schema | 0.12.0  |  44145
 searchspot          | 0.15.2  |  62618
 diesel_infer_schema | 0.14.0  |  54552
 diesel_infer_schema | 0.16.0  |  59094
 searchspot          | 0.14.0  |  59745
 diesel_infer_schema | 0.15.0  |  56265
 searchspot          | 0.15.0  |  62500
 searchspot          | 0.15.1  |  62603
 ipfsapi             | 0.1.6   |  68582
 searchspot          | 0.15.4  |  63062
 ipfsapi             | 0.1.3   |  64146
 ipfsapi             | 0.1.5   |  64240
 ipfsapi             | 0.1.0   |  63153
 ipfsapi             | 0.1.1   |  63479
 ipfsapi             | 0.1.2   |  64075
 ipfsapi             | 0.1.4   |  64157
 pom                 | 2.0.0   |  64739
 ipfsapi             | 0.1.7   |  68726
 fantoccini          | 0.8.1   |  77851
 fantoccini          | 0.8.0   |  71985
 searchspot          | 0.16.0  |  72539
 semverver           | 0.1.0   |  74641
 semverver           | 0.1.1   |  75472
 swc_ecma_parser     | 0.8.0   | 123132
 infer_schema_macros | 1.4.0   | 124335
 infer_schema_macros | 1.4.0   | 124335
 amadeus-parquet     | 0.1.3   | 176821
 amadeus-parquet     | 0.1.4   | 180078
 hcs-rs              | 0.2.1   | 188689
 hcs-rs              | 0.2.0   | 188669
(33 rows)
