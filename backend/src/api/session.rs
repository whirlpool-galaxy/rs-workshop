use sqlx::{query_as, Error, FromRow, Pool, Postgres};

#[derive(FromRow)]
pub struct Session {
    pub key: Vec<u8>,
    pub val: String,
}

pub enum SessionState {
    SignedOut,
    SignedIn(String),
}

pub async fn check_session(db: Pool<Postgres>, ssid: [u8; 16]) -> Result<SessionState, Error> {
    let session = query_as!(Session, "SELECT * FROM session WHERE key = $1;", &ssid)
        .fetch_optional(&db)
        .await?;

    match session {
        Some(s) => Ok(SessionState::SignedIn(s.val)),
        None => Ok(SessionState::SignedOut),
    }
}
