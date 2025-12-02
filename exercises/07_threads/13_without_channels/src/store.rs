use std::collections::BTreeMap;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::data::{Status, Ticket, TicketDraft};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TicketId(u64);

#[derive(Default)]
pub struct InnerStore {
    tickets: BTreeMap<TicketId, Arc<RwLock<Ticket>>>,
    counter: u64,
}

#[derive(Clone, Default)]
pub struct TicketStore {
    inner: Arc<RwLock<InnerStore>>,
}

impl TicketStore {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn write(
        &self,
    ) -> Result<RwLockWriteGuard<InnerStore>, std::sync::PoisonError<RwLockWriteGuard<InnerStore>>>
    {
        self.inner.write()
    }

    pub fn read(
        &self,
    ) -> Result<RwLockReadGuard<InnerStore>, std::sync::PoisonError<RwLockReadGuard<InnerStore>>>
    {
        self.inner.read()
    }
}

impl InnerStore {
    pub fn add_ticket(&mut self, ticket: TicketDraft) -> TicketId {
        let id = TicketId(self.counter);
        self.counter += 1;
        let ticket = Ticket {
            id,
            title: ticket.title,
            description: ticket.description,
            status: Status::ToDo,
        };
        let ticket = Arc::new(RwLock::new(ticket));
        self.tickets.insert(id, ticket);
        id
    }

    pub fn get(&self, id: TicketId) -> Option<Arc<RwLock<Ticket>>> {
        self.tickets.get(&id).cloned()
    }
}
