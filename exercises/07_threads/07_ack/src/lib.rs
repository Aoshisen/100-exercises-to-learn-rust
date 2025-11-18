use crate::{
    data::{Ticket, TicketDraft},
    store::{TicketId, TicketStore},
};
use std::sync::mpsc::{channel, Receiver, Sender};

pub mod data;
pub mod store;

// Refer to the tests to understand the expected schema.
pub enum Command {
    Insert {
        draft: TicketDraft,
        response_sender: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_sender: Sender<Option<Ticket>>,
    },
}

pub fn launch() -> Sender<Command> {
    let (sender, receiver) = channel();
    std::thread::spawn(move || server(receiver));
    //sender 被返回出去了,receiver 被 移动给了server 函数
    sender
}

// TODO: handle incoming commands as expected.
pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    // 循环的等待receiver 接受到消息 然后做对应的处理
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_sender,
            }) => {
                //1.通过store 添加ticket;
                let id = store.add_ticket(draft);
                //2. 通过response_sender 发送返回值
                let _ = response_sender.send(id);
            }
            Ok(Command::Get {
                id,
                response_sender,
            }) => {
                let ticket = store.get(id).cloned();
                let _ = response_sender.send(ticket);
            }

            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
