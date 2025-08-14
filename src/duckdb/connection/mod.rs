mod transaction;

use std::fmt::Debug;
use diesel::{Connection, ConnectionError, ConnectionResult, QueryResult};
use diesel::connection::{ConnectionSealed, Instrumentation, SimpleConnection, TransactionManager};
use diesel::query_builder::{QueryFragment, QueryBuilder, QueryId};
use diesel::result::Error;
use crate::DuckDB;
use crate::duckdb::connection::transaction::DuckDBTransactionManager;
use crate::duckdb::query_builder::DuckDBQueryBuilder;

pub struct DuckDBConnection {
    pub raw: duckdb::Connection,
}

impl ConnectionSealed for DuckDBConnection {}

impl SimpleConnection for DuckDBConnection {
    fn batch_execute(&mut self, query: &str) -> QueryResult<()> {
        todo!()
    }
}

struct ErrorAdapter(duckdb::Error);

impl From<duckdb::Error> for ErrorAdapter {
    fn from(error: duckdb::Error) -> Self {
        Self(error)
    }
}

impl From<ErrorAdapter> for diesel::result::Error {
    fn from(value: ErrorAdapter) -> Self {
        todo!()
    }
}

impl Connection for DuckDBConnection {
    type Backend = DuckDB;
    type TransactionManager = DuckDBTransactionManager;

    fn establish(database_url: &str) -> ConnectionResult<Self> {
        let raw = Self::inner_establish(database_url)?;

        Ok(Self {
            raw
        })
    }

    fn transaction<T, E, F>(&mut self, f: F) -> Result<T, E>
    where
        F: FnOnce(&mut Self) -> Result<T, E>,
        E: From<Error>
    {
        todo!()
    }

    fn begin_test_transaction(&mut self) -> QueryResult<()> {
        todo!()
    }

    fn test_transaction<T, E, F>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut Self) -> Result<T, E>,
        E: Debug
    {
        todo!()
    }

    fn execute_returning_count<T>(&mut self, source: &T) -> QueryResult<usize>
    where
        T: QueryFragment<Self::Backend> + QueryId
    {
        let mut qb = DuckDBQueryBuilder::default();
        source.to_sql(&mut qb, &DuckDB)?;

        let conn = &self.raw;
        let sql = qb.finish();

        let res = conn.execute(sql.as_str(), []);
        res.map_err(ErrorAdapter::from).map_err(|e| e.into())
    }

    fn transaction_state(&mut self) -> &mut <Self::TransactionManager as TransactionManager<Self>>::TransactionStateData {
        todo!()
    }

    fn instrumentation(&mut self) -> &mut dyn Instrumentation {
        todo!()
    }

    fn set_instrumentation(&mut self, instrumentation: impl Instrumentation) {
        todo!()
    }
}


impl DuckDBConnection {
    fn inner_establish(database_url: &str) -> Result<duckdb::Connection, ConnectionError> {
        let raw = duckdb::Connection::open(database_url)
            .map_err(ErrorAdapter::from)
            .map_err(|e| ConnectionError::CouldntSetupConfiguration(e.into()))?;

        Ok(raw)
    }
}