// TODO: Replace `todo!()`s with the correct implementation.
//  Implement `IntoIterator` for `&TicketStore`. The iterator should yield immutable
//  references to the tickets, ordered by their `TicketId`.
//  Implement additional traits on `TicketId` if needed.

use std::collections::BTreeMap;
use std::collections::btree_map::{ IntoValues, Values };
use std::ops::{ Index, IndexMut };
use ticket_fields::{ TicketDescription, TicketTitle };

#[derive(Clone)]
pub struct TicketStore {
    tickets: BTreeMap<TicketId, Ticket>,
    counter: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct TicketId(u64);

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: BTreeMap::default(),
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
}

impl Index<TicketId> for TicketStore {
    type Output = Ticket;

    fn index(&self, index: TicketId) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl Index<&TicketId> for TicketStore {
    type Output = Ticket;

    fn index(&self, index: &TicketId) -> &Self::Output {
        &self[*index]
    }
}

impl IndexMut<TicketId> for TicketStore {
    fn index_mut(&mut self, index: TicketId) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl IndexMut<&TicketId> for TicketStore {
    fn index_mut(&mut self, index: &TicketId) -> &mut Self::Output {
        &mut self[*index]
    }
}

//需要用到生命周期 并且只是借用不会 发生所有权的转移
// 实现IntoIterator 三问
// 1. type Item 填什么
//   1.1: 如果想拿走所有权 type Item =Ticket
//   1.2 如果只是想看一下 type Item =&'a Ticket <需要指定生命周期>
//   1.3 如果想要修改他 type Item=&'a mut Ticket <需要指定生命周期> 来保证 不会出现悬垂引用和 double drop

//2. type IntoIter 填什么
// 2.1: 我从什么数据结构里面抽取的迭代器
// 2.2: 如果BTreeMap into_values 那么就是IntoValues IntoValues<TicketId, Ticket>
//2.3 如果是&BTreeMap 调用values 那么就是 Values. Values<'a, TicketId, Ticket>
//2.3 如果是&mut BTreeMap 调用.values_mut 那么就是ValuesMut ValuesMut<'a, TicketId, Ticket>
//

impl<'a> IntoIterator for &'a TicketStore {
    type Item = &'a Ticket;
    type IntoIter = Values<'a, TicketId, Ticket>;
    fn into_iter(self) -> Self::IntoIter {
        self.tickets.values().into_iter()
    }
}

//不需要用到生命周期 会发生所有权转移
impl IntoIterator for TicketStore {
    type Item = Ticket;
    type IntoIter = IntoValues<TicketId, Ticket>;
    fn into_iter(self) -> Self::IntoIter {
        self.tickets.into_values()
    }
}
#[cfg(test)]
mod tests {
    use crate::{ Status, TicketDraft, TicketId, TicketStore };
    use ticket_fields::test_helpers::{ ticket_description, ticket_title };

    #[test]
    fn works() {
        let mut store = TicketStore::new();

        let n_tickets = 5;

        for i in 0..n_tickets {
            let draft = TicketDraft {
                title: ticket_title(),
                description: ticket_description(),
            };
            let id = store.add_ticket(draft.clone());
            let ticket = &store[id];
            assert_eq!(draft.title, ticket.title);
            assert_eq!(draft.description, ticket.description);
            assert_eq!(ticket.status, Status::ToDo);

            let ticket = &mut store[id];
            ticket.status = Status::InProgress;

            let ticket = &store[id];
            assert_eq!(ticket.status, Status::InProgress);
        }

        let ids: Vec<TicketId> = (&store)
            .into_iter()
            .map(|t| t.id)
            .collect();
        let sorted_ids = {
            let mut v = ids.clone();
            v.sort();
            v
        };
        assert_eq!(ids, sorted_ids);
    }
}
