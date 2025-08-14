use diesel::backend::{Backend, SqlDialect, TrustedBackend};
use diesel::query_builder::bind_collector::RawBytesBindCollector;
use diesel::sql_types::TypeMetadata;
use crate::duckdb::query_builder::DuckDBQueryBuilder;
use crate::duckdb::types::value::DuckDBRawValue;
use crate::DuckDBTypeMetadata;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
pub struct DuckDB;

impl TrustedBackend for DuckDB {}
impl Backend for DuckDB {
    type QueryBuilder = DuckDBQueryBuilder;
    type RawValue<'a> = DuckDBRawValue<'a>;
    type BindCollector<'a> = RawBytesBindCollector<Self>;
}

impl TypeMetadata for DuckDB {
    type TypeMetadata = DuckDBTypeMetadata;

    // This probably needs to be something else, like in Pg. See:
    // https://duckdb.org/docs/stable/sql/statements/create_type.html
    type MetadataLookup = ();
}

impl SqlDialect for DuckDB {
    type ReturningClause = ();
    type OnConflictClause = ();
    type InsertWithDefaultKeyword = ();
    type BatchInsertSupport = ();
    type ConcatClause = ();
    type DefaultValueClauseForInsert = ();
    type EmptyFromClauseSyntax = ();
    type ExistsSyntax = ();
    type ArrayComparison = ();
    type SelectStatementSyntax = ();
    type AliasSyntax = ();
}
