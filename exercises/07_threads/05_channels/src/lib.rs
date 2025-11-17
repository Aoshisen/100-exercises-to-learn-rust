use std::sync::mpsc::{Receiver, Sender};

use crate::data::TicketDraft;

pub mod data;
pub mod store;

pub enum Command {
    Insert(TicketDraft),
}

// Start the system by spawning the server thread.
// It returns a `Sender` instance which can then be used
// by one or more clients to interact with the server.
pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: The server task should **never** stop.
//  Enter a loop: wait for a command to show up in
//  the channel, then execute it, then start waiting
//  for the next command.
pub fn server(receiver: Receiver<Command>) {
    let mut s = store::TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(command) => match command {
                Command::Insert(ticket) => {
                    s.add_ticket(ticket);
                }
            },
            Err(_) => {
                // 通道关闭，但根据要求服务器不应停止
                // 可以选择继续循环或记录日志
                continue; // 或者使用 std::thread::sleep(Duration::from_millis(100));
            }
        }
    }
}
