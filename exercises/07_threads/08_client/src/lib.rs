use crate::data::{ Ticket, TicketDraft };
use crate::store::{ TicketId, TicketStore };
use std::sync::mpsc::{ self, Receiver, Sender };

pub mod data;
pub mod store;

enum Command {
    Insert {
        draft: TicketDraft,
        // One-shot channel owned by the caller, so every request gets its own reply.
        response_sender: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_sender: Sender<Option<Ticket>>,
    },
}

/// A handle to the ticket store.
///
/// Cheap to clone: every clone talks to the same background thread over the
/// same channel, so the store is still accessed from a single place.
#[derive(Clone)]
pub struct TicketStoreClient {
    sender: Sender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> TicketId {
        let (response_sender, response_receiver) = mpsc::channel();
        self.sender
            .send(Command::Insert {
                draft,
                response_sender,
            })
            .expect("the store thread has shut down");
        response_receiver.recv().expect("the store thread dropped the response channel")
    }

    pub fn get(&self, id: TicketId) -> Option<Ticket> {
        let (response_sender, response_receiver) = mpsc::channel();
        self.sender
            .send(Command::Get { id, response_sender })
            .expect("the store thread has shut down");
        response_receiver.recv().expect("the store thread dropped the response channel")
    }
}

pub fn launch() -> TicketStoreClient {
    let (sender, receiver) = mpsc::channel();
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender }
}

fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    // `recv` returns `Err` once every `Sender` is dropped, which is how the
    // thread shuts down when the last client goes away.
    while let Ok(command) = receiver.recv() {
        match command {
            Command::Insert { draft, response_sender } => {
                let ticket_id = store.add_ticket(draft);
                let _ = response_sender.send(ticket_id);
            }
            Command::Get { id, response_sender } => {
                // The borrow of `store` cannot cross the channel; send an owned copy.
                let ticket = store.get(id).cloned();
                let _ = response_sender.send(ticket);
            }
        }
    }
}
