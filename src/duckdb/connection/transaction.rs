use diesel::connection::{TransactionManager, TransactionManagerStatus};
use diesel::QueryResult;
use diesel::result::Error;
use crate::duckdb::connection::DuckDBConnection;

#[derive(Default)]
pub struct DuckDBTransactionManager {

}

impl TransactionManager<DuckDBConnection> for DuckDBTransactionManager {
    type TransactionStateData = Self;

    fn begin_transaction(conn: &mut DuckDBConnection) -> QueryResult<()> {
        todo!()
    }

    fn rollback_transaction(conn: &mut DuckDBConnection) -> QueryResult<()> {
        todo!()
    }

    fn commit_transaction(conn: &mut DuckDBConnection) -> QueryResult<()> {
        todo!()
    }

    fn transaction_manager_status_mut(conn: &mut DuckDBConnection) -> &mut TransactionManagerStatus {
        todo!()
    }

    fn transaction<F, R, E>(conn: &mut DuckDBConnection, callback: F) -> Result<R, E>
    where
        F: FnOnce(&mut DuckDBConnection) -> Result<R, E>,
        E: From<Error>
    {
        todo!()
    }

    fn is_broken_transaction_manager(conn: &mut DuckDBConnection) -> bool {
        todo!()
    }
}