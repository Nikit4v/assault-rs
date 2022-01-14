use std::thread;
use std::thread::Thread;
use crate::render::action::Action;
use crate::render::backend::Backend;
use crate::render::threading::Deque;


pub struct Surface<'a> {
    backend: &'a dyn Backend,
    actions_to_apply: Deque<dyn Action>,
    action_worker: &'a Thread
}

impl<'a> Surface<'a> {
    pub fn new(backend: &'a dyn Backend) -> Self {
        let worker_handler= thread::spawn(Self::worker);

        Self {
            backend,
            actions_to_apply: Deque::new(),
            action_worker: worker_handler.thread()
        }
    }


    pub fn apply_action(&mut self, action: &'a dyn Action) {
        self.actions_to_apply.schedule(Box::new(action))
    }


    fn worker(&mut self) {
        let action = self.actions_to_apply.accept_one();
        action.apply()
    }
}

