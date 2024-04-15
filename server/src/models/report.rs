use edgedb_tokio::Queryable;

use super::Reason;

#[derive(Debug, Queryable)]
pub struct Report {
    pub reason: Reason,
}
