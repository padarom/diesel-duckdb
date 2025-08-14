use diesel::query_builder::QueryBuilder;
use diesel::QueryResult;
use crate::DuckDB;

pub struct DuckDBQueryBuilder;

impl QueryBuilder<DuckDB> for DuckDBQueryBuilder {
    fn push_sql(&mut self, sql: &str) {
        todo!()
    }

    fn push_identifier(&mut self, identifier: &str) -> QueryResult<()> {
        todo!()
    }

    fn push_bind_param(&mut self) {
        todo!()
    }

    fn push_bind_param_value_only(&mut self) {
        todo!()
    }

    fn finish(self) -> String {
        todo!()
    }
}