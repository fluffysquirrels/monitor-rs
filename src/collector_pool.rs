use crate::{
    BoxError,
    collector::collector_client::CollectorClient
};
use tokio::sync::Mutex;

pub type Client = CollectorClient<tonic::transport::Channel>;

pub struct Pool {
    state: Mutex<State>,
    endpoint: tonic::transport::Endpoint,
    url: String,
}

struct State {
    conn: Option<Client>,
    token: usize,
}

pub struct PoolClient {
    conn: Client,
    token: usize,
}

impl Pool {
    pub fn new(url: &str, endpoint: tonic::transport::Endpoint) -> Pool {
        Pool {
            state: Mutex::new(State {
                conn: None,
                token: 0,
            }),
            endpoint,
            url: url.to_owned(),
        }
    }

    pub async fn get(&self) -> Result<PoolClient, BoxError> {
        let mut lock_state = self.state.lock().await;
        if let Some(c) = &lock_state.conn {
            debug!("{} Re-using pool connection", self.url);
            return Ok(PoolClient {
                conn: c.clone(),
                token: lock_state.token,
            });
        }

        // No current connection, make one.
        debug!("{} Starting new pool connection", self.url);
        let c = match Client::connect(self.endpoint.clone()).await {
            Err(e) => {
                error!("{} connect error: {}", self.url, e);
                return Err(e.into());
            },
            Ok(c) => c,
        };
        debug!("{} Connected", self.url);

        lock_state.conn = Some(c.clone());
        Ok(PoolClient {
            conn: c,
            token: lock_state.token,
        })
    }

    /// Return a PoolClient that has faulted (returned an error).
    /// The underlying connection will not be returned by `get` again.
    pub async fn discard_faulted(&self, client: PoolClient) {
        debug!("{} Discarding faulted connection", self.url);
        let mut lock_state = self.state.lock().await;

        // If the current connection failed (as determined by token)
        if client.token == lock_state.token {
            debug!("{} First time discarding this faulted connection", self.url);
            // Start a new connection and update token so we don't discard the new one.
            lock_state.token = lock_state.token.overflowing_add(1).0;
            lock_state.conn = None;
        }
    }
}

impl PoolClient {
    pub fn get(&mut self) -> &mut Client {
        &mut self.conn
    }
}
