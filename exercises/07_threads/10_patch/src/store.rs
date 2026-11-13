use crate::data::{ Status, Ticket, TicketDraft, TicketPatch };
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TicketId(u64);

#[derive(Clone)]
pub struct TicketStore {
    tickets: BTreeMap<TicketId, Ticket>,
    counter: u64,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: BTreeMap::new(),
            counter: 0,
        }
    }

    pub fn add_ticket(&mut self, ticket: TicketDraft) -> TicketId {
        let id = TicketId(self.counter);
        self.counter += 1;
        let ticket = Ticket {
            id,
            title: ticket.title,
            description: ticket.description,
            status: Status::ToDo,
        };
        self.tickets.insert(id, ticket);
        id
    }

    pub fn get(&self, id: TicketId) -> Option<&Ticket> {
        self.tickets.get(&id)
    }

    pub fn get_mut(&mut self, id: TicketId) -> Option<&mut Ticket> {
        self.tickets.get_mut(&id)
    }
    /// 用 patch 中出现的字段覆盖自身，`None` 字段保持不变。
    pub fn apply_patch(&mut self, patch: TicketPatch) -> Option<()> {
        let mut_ticket = self.get_mut(patch.id)?;
        if let Some(status) = patch.status {
            mut_ticket.status = status;
        }
        if let Some(title) = patch.title {
            mut_ticket.title = title;
        }
        if let Some(description) = patch.description {
            mut_ticket.description = description;
        }
        Some(())
    }
}
