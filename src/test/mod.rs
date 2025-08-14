use diesel::{RunQueryDsl, Connection, sql_query, ConnectionResult};
use crate::duckdb::connection::DuckDBConnection;

const CREATE_TEST_TABLE: &str = "CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT)";

fn conn() -> ConnectionResult<DuckDBConnection> {
    DuckDBConnection::establish(":memory:")
}

#[test]
fn connect() {
    let conn = conn();

    assert!(conn.is_ok());
}

#[test]
fn create_test_table() {
    let mut conn = conn().unwrap();
    let ret = sql_query(CREATE_TEST_TABLE).execute(&mut conn);

    assert!(ret.is_ok());

    // For now. This should fail due to table "tests" not existing.
    let res = conn.raw.execute("SELECT * FROM tests;", []);
    println!("{:?}", res);
    assert!(res.is_ok());
}