
use core::sync::atomic::{AtomicBool, Ordering, AtomicUsize};
use kernel_hw_cpu::{ArchCpu, Cpu};
use spin::{Once, Mutex};

use crate::atomic::AtomicBaseTypeExtensions;

pub struct Waiter {
    cond: Once<bool>,
}

impl Waiter {
    pub const fn new() -> Self {
        Self {
            cond: Once::new()
        }
    }

    pub const fn new_notified() -> Self {
        Self {
            cond: Once::initialized(true),
        }
    }

    pub fn wait(&self) {
        self.cond.wait();
    }

    pub fn notify(&self) {
        self.cond.call_once(|| true);
    }
}

unsafe impl Sync for Waiter {}

pub struct RatchetingWaiter {
    notified_ticket: AtomicUsize,
    waited_ticket: AtomicUsize,
    next_ticket: AtomicUsize,
}

impl RatchetingWaiter {
    pub const fn new() -> Self {
        Self {
            notified_ticket: AtomicUsize::new(0),
            waited_ticket: AtomicUsize::new(0),
            next_ticket: AtomicUsize::new(1),
        }
    }

    pub fn wait(&self) {
        let ticket = self.next_ticket.fetch_add(1, Ordering::Relaxed);
        loop {
            while self.notified_ticket.get() != ticket {
                ArchCpu::spin_pause()
            }
            if self.notified_ticket.load(Ordering::Relaxed) == ticket {
                break
            }
        }
        // Our ticket has been notified. Increase the waited ticket to notify the notifier about the successful wait
        self.waited_ticket.store(ticket, Ordering::Relaxed);
    }

    pub fn notify_one(&self) {
        // Wait for the last ticket notification to be acknowledged
        loop {
            while self.waited_ticket.get() != self.notified_ticket.get() {
                ArchCpu::spin_pause()
            }
            if self.waited_ticket.load(Ordering::Relaxed) == self.notified_ticket.load(Ordering::Relaxed) {
                break
            }
        }
        self.notified_ticket.fetch_add(1, Ordering::SeqCst);
    }
}

pub struct CountWaiter {
    remaining: AtomicUsize,
}

impl CountWaiter {
    pub const fn new(count: usize) -> Self {
        Self {
            remaining: AtomicUsize::new(count),
        }
    }

    pub fn wait(&self) {
        while self.remaining.load(Ordering::Relaxed) != 0 {}
    }

    pub fn decrement(&self) {
        self.remaining.fetch_sub(1, Ordering::Relaxed);
    }

    pub fn set_count(&self, count: usize) {
        self.remaining.store(count, Ordering::Relaxed);
    }
}

pub struct Semaphore {
    remaining_tickets: Mutex<usize>,
    max_tickets: usize,
}

impl Semaphore {

    pub const fn new(max_tickets: usize) -> Self {
        Self {
            remaining_tickets: Mutex::new(max_tickets),
            max_tickets,
        }
    }

    pub fn unlock(&self) -> usize {
        let mut remaining_tickets = self.remaining_tickets.lock();
        if *remaining_tickets == 0 {
            panic!("Semaphore unlocked too often");
        }
        *remaining_tickets += 1;
        *remaining_tickets
    }

    pub fn lock(&self) {
        while {
            let mut remaining_tickets = self.remaining_tickets.lock();
            if *remaining_tickets < self.max_tickets {
                *remaining_tickets -= 1;
            }
            *remaining_tickets
        } >= self.max_tickets {};
    }

}

pub struct RendezvousChannel<T : Copy> {
    consume_waiter: RatchetingWaiter,
    submit_waiter: RatchetingWaiter,
    value_holder: Mutex<Option<T>>,
    first_time: AtomicBool,
}

impl<T : Copy> RendezvousChannel<T> {
    pub const fn new() -> Self {
        Self {
            consume_waiter: RatchetingWaiter::new(),
            submit_waiter: RatchetingWaiter::new(),
            value_holder: Mutex::new(None),
            first_time: AtomicBool::new(true),
        }
    }

    pub fn consume(&self) -> T {
        let value = {
            self.consume_waiter.wait();
            let value_holder = self.value_holder.lock();
            value_holder.unwrap()
        };

        self.submit_waiter.notify_one();
        value
    }

    pub fn submit(&self, value: T) {
        {
            if !self.first_time.swap(false, Ordering::Relaxed) {
                self.submit_waiter.wait();
            }
            let mut value_holder = self.value_holder.lock();
            *value_holder = Some(value);
        }

        self.consume_waiter.notify_one();
    }
}
