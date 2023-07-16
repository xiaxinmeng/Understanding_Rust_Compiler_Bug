
error[E0053]: method `metadata` has an incompatible type for trait
   --> diesel/src/pg/types/array.rs:14:25
    |
14  |     fn metadata(lookup: &PgMetadataLookup) -> PgTypeMetadata {
    |                         ^^^^^^^^^^^^^^^^^ expected associated type, found struct `pg::metadata_lookup::PgMetadataLookup`
    | 
   ::: diesel/src/sql_types/mod.rs:402:25
    |
402 |     fn metadata(lookup: &Self::MetadataLookup) -> Self::TypeMetadata;
    |                         --------------------- type in trait
    |
    = note: expected type `fn(&<pg::backend::Pg as sql_types::TypeMetadata>::MetadataLookup) -> <pg::backend::Pg as sql_types::TypeMetadata>::TypeMetadata`
               found type `fn(&pg::metadata_lookup::PgMetadataLookup) -> pg::backend::PgTypeMetadata`
