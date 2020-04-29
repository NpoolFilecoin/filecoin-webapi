use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{Receiver, TryRecvError};
use std::sync::Mutex;
use std::thread::JoinHandle;

lazy_static! {
    static ref WORKER_TOKEN: AtomicU64 = AtomicU64::new(0);
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PollingState {
    Started(u64),
    Pending,
    Done(Value),
    Error,
}

type WorkerReceiver = Receiver<Value>;

pub struct ServState {
    workers: HashMap<u64, (JoinHandle<()>, WorkerReceiver)>,
}

impl ServState {
    pub fn new() -> Self {
        Self {
            workers: HashMap::new(),
        }
    }

    pub fn enqueue(&mut self, handle: JoinHandle<()>, rx: WorkerReceiver) -> PollingState {
        let token = WORKER_TOKEN.fetch_add(1, Ordering::SeqCst);
        self.workers.insert(token, (handle, rx));

        PollingState::Started(token)
    }

    // TODO: remove if worker is done
    // TODO: remove if not query after long time
    pub fn get(&self, token: u64) -> Option<PollingState> {
        self.workers.get(&token).map(|x| match x.1.try_recv() {
            Ok(r) => PollingState::Done(r),
            Err(TryRecvError::Empty) => PollingState::Pending,
            Err(TryRecvError::Disconnected) => PollingState::Error,
        })
    }
}
