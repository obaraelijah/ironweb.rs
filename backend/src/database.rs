use actix::prelude::*;
use anyhow::Result;
use diesel::{
    delete, insert_into,
    r2d2::{ConnectionManager, Pool},
    update, PgConnection,
};
use webapp::schema::sessions::dsl::*;

/// The database executor actor
pub struct DatabaseExecutor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DatabaseExecutor {
    type Context = SyncContext<Self>;
}

// Todo -> ALl Handlers + their impl(delte, update, + create sessiom)