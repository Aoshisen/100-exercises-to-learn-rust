use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{self, Receiver, SyncSender};

pub mod data;
pub mod store;

enum Command {
    Insert {
        draft: TicketDraft,
        // The reply channel is unbounded on purpose: the server must never
        // block while answering, otherwise a full queue would deadlock it.
        response_channel: mpsc::Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: mpsc::Sender<Option<Ticket>>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum ClientError {
    /// The background thread has shut down.
    ServerGone,
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::ServerGone => f.write_str("the store thread has shut down"),
        }
    }
}

impl std::error::Error for ClientError {}

/// A handle to the ticket store.
///
/// The channel to the server is bounded: when the queue is full, `insert` and
/// `get` block until the server catches up, propagating backpressure instead of
/// letting requests pile up in memory.
#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, ClientError> {
        let (response_sender, response_receiver) = mpsc::channel();
        self.sender
            .send(Command::Insert {
                draft,
                response_channel: response_sender,
            })
            .map_err(|_| ClientError::ServerGone)?;
        response_receiver
            .recv()
            .map_err(|_| ClientError::ServerGone)
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, ClientError> {
        let (response_sender, response_receiver) = mpsc::channel();
        self.sender
            .send(Command::Get {
                id,
                response_channel: response_sender,
            })
            .map_err(|_| ClientError::ServerGone)?;
        response_receiver
            .recv()
            .map_err(|_| ClientError::ServerGone)
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    assert!(capacity > 0, "a bounded channel needs a non-zero capacity");
    let (sender, receiver) = mpsc::sync_channel(capacity);
    std::thread::spawn(move || server(receiver));

    TicketStoreClient { sender }
}

fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    // `recv` returns `Err` once every sender is dropped, which is how the
    // thread shuts down when the last client goes away.
    while let Ok(command) = receiver.recv() {
        match command {
            Command::Insert {
                draft,
                response_channel,
            } => {
                let _ = response_channel.send(store.add_ticket(draft));
            }
            Command::Get {
                id,
                response_channel,
            } => {
                // The borrow of `store` cannot cross the channel; send an owned copy.
                let _ = response_channel.send(store.get(id).cloned());
            }
        }
    }
}
