use crate::collector::collector_client::CollectorClient;
use tokio::sync::Mutex;

pub type Client = CollectorClient<tonic::transport::Channel>;

pub struct Pool {
    state: Mutex<State>,
    endpoint: tonic::transport::Endpoint,
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
    pub fn new(endpoint: tonic::transport::Endpoint) -> Pool {
        Pool {
            state: Mutex::new(State {
                conn: None,
                token: 0,
            }),
            endpoint,
        }
    }

    pub async fn get(&self) -> Result<PoolClient, Box<dyn std::error::Error + Send + Sync>> {
        let mut lock_state = self.state.lock().await;
        if let Some(c) = &lock_state.conn {
            trace!("Re-using pool connection");
            return Ok(PoolClient {
                conn: c.clone(),
                token: lock_state.token,
            });
        }

        // No current connection, make one.
        trace!("Starting new pool connection");
        let c = Client::connect(self.endpoint.clone()).await?;
        lock_state.conn = Some(c.clone());
        Ok(PoolClient {
            conn: c,
            token: lock_state.token,
        })
    }

    /// Return a PoolClient that has faulted (returned an error).
    /// The underlying connection will not be returned by `get` again.
    pub async fn discard_faulted(&self, client: PoolClient) {
        trace!("Discarding faulted connection");
        let mut lock_state = self.state.lock().await;

        // If the current connection failed (as determined by token)
        if client.token == lock_state.token {
            trace!("First time discarding this faulted connection");
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
