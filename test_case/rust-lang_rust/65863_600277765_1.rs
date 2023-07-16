
cratesfyi=> select name, version, releases.id from builds inner join releases on releases.id = builds.rid inner join crates on crates.id = releases.crate_id where output like '%is not implemented for `()`%';
         name          |    version    |   id   
-----------------------+---------------+--------
 reducer               | 1.1.0         | 125897
 amadeus-parquet       | 0.1.1         | 169206
 amadeus-postgres      | 0.1.3         | 176820
 amadeus-parquet       | 0.1.2         | 169266
 yukikaze              | 1.0.0-alpha.5 | 169612
 amadeus-parquet       | 0.1.3         | 176821
 amadeus-commoncrawl   | 0.1.3         | 176822
 amadeus-serde         | 0.1.3         | 176824
 amadeus-aws           | 0.1.3         | 176826
 vicuna                | 0.1.2         | 189614
 amadeus-postgres      | 0.1.4         | 180077
 amadeus-parquet       | 0.1.4         | 180078
 amadeus-commoncrawl   | 0.1.4         | 180079
 amadeus-serde         | 0.1.4         | 180080
 amadeus-aws           | 0.1.4         | 180081
 vicuna                | 0.1.1         | 189190
 amadeus-aws           | 0.1.5         | 189240
 amadeus-parquet       | 0.1.5         | 189242
 nu                    | 0.6.1         | 186499
 vicuna                | 0.1.0         | 189185
 amadeus-serde         | 0.1.5         | 189233
 amadeus-postgres      | 0.1.5         | 189235
 amadeus-commoncrawl   | 0.1.5         | 189237
 vicuna                | 0.1.3         | 189615
 xtra                  | 0.2.1         | 199674
 wasm-reader           | 0.1.0         | 198978
 amadeus-parquet       | 0.1.6         | 193727
 wasm-reader           | 0.1.1         | 200658
 cpp_to_rust_generator | 0.0.0         |  38434
 noria                 | 0.4.0         | 206426
 noria-server          | 0.4.0         | 206463
 noria                 | 0.4.1         | 208596
 wasm-reader           | 0.2.0         | 207153
(33 rows)
