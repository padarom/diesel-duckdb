use diesel::query_builder::QueryBuilder;
use diesel::QueryResult;
use crate::DuckDB;

#[derive(Default)]
pub struct DuckDBQueryBuilder {
    sql: String,
    bind_idx: u32,
}

impl QueryBuilder<DuckDB> for DuckDBQueryBuilder {
    fn push_sql(&mut self, sql: &str) {
        self.sql.push_str(sql);
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
        self.sql
    }
}