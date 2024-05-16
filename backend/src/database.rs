use actix::prelude::*;
use anyhow::Result;
use diesel::{
    delete, insert_into,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
    update,
};
use log::debug;
use webapp::{protocol::model::Session, schema::sessions::dsl::*};

/// The database executor actor
pub struct DatabaseExecutor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DatabaseExecutor {
    type Context = SyncContext<Self>;
}

/// The create session message
pub struct CreateSession(pub String);

impl Message for CreateSession {
    type Result = Result<Session>;
}

impl Handler<CreateSession> for DatabaseExecutor {
    type Result = Result<Session>;

    fn handle(&mut self, msg: CreateSession, _: &mut Self::Context) -> Self::Result {
        // Insert the session into the database
        debug!("Creating new session: {}", msg.0);
        let conn = &mut *self.0.get()?;
        Ok(insert_into(sessions)
            .values(&Session::new(msg.0))
            .get_result::<Session>(conn)?)
    }
}

// Update session message
pub struct UpdateSession {
    /// old session token
    pub old_token: String,

    /// new session token
    pub new_token: String,
}

impl Message for UpdateSession {
    type Result = Result<Session>;
}

impl Handler<UpdateSession> for DatabaseExecutor {
    type Result = Result<Session>;

    fn handle(&mut self, msg: UpdateSession, _: &mut Self::Context) -> Self::Result {
        // Update the session
        debug!("Updating session: {}", msg.old_token);
        let conn = &mut *self.0.get()?;
        Ok(update(sessions.filter(token.eq(&msg.old_token)))
            .set(token.eq(&msg.new_token))
            .get_result::<Session>(conn)?)
    }
}

/// Delete session + needs a token
pub struct DeleteSession(pub String);
 