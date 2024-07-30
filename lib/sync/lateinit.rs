
use alloc::sync::Arc;
use spin::RwLock;

use crate::waiter::Waiter;

pub struct LateInitArc<T> {
    value: RwLock<Option<Arc<T>>>,
    waiter: Waiter,
}

impl<T> LateInitArc<T> {
    pub const fn new() -> Self {
        let new = Self {
            value: RwLock::new(None),
            waiter: Waiter::new(),
        };

        new
    }

    pub fn try_set(&self, v: T) -> Result<(), &'static str> {
        let mut value = self.value.write();
        if value.is_some() {
            return Err("Can't set value in LateInitArc: is already initialized");
        }
        *value = Some(Arc::new(v));
        self.waiter.notify();
        Ok(())
    }

    pub fn set(&self, v: T) {
        self.try_set(v).unwrap();
    }

    pub fn get(&self) -> Arc<T> {
        self.value.read().as_ref().unwrap().clone()
    }

    pub fn has_value(&self) -> bool {
        let value = self.value.read();
        value.is_some()
    }

    pub fn try_get(&self) -> Result<Arc<T>, &'static str> {
        let value = self.value.read();
        if value.is_none() {
            return Err("Can't get value in LateInitArc: is not initialized");
        }
        Ok(value.as_ref().unwrap().clone())
    }

    pub fn wait_for_init(&self) {
        self.waiter.wait()
    }

}

unsafe impl<T: Sync + Send> Send for LateInitArc<T> {}
unsafe impl<T: Sync + Send> Sync for LateInitArc<T> {}

pub struct LateInit<T : Sync + Send + Clone> {
    value: RwLock<Option<T>>
}

impl<T : Sync + Send + Clone> LateInit<T> {
    pub fn new() -> Self {
        Self {
            value: RwLock::new(None),
        }
    }

    pub fn try_set(&self, v: T) -> Result<(), &'static str> {
        let mut value = self.value.write();
        if value.is_some() {
            return Err("Can't set value in LateInitArc: is already initialized");
        }
        *value = Some(v);
        Ok(())
    }

    pub fn set(&self, v: T) {
        self.try_set(v).ok();
    }

    pub fn get(&self) -> T {
        self.value.read().as_ref().unwrap().clone()
    }

    pub fn try_get(&self) -> Result<T, &'static str> {
        let value = self.value.read();
        if value.is_none() {
            return Err("Can't get value in LateInitArc: is not initialized");
        }
        Ok(value.as_ref().unwrap().clone())
    }

}

unsafe impl<T : Sync + Send + Clone> Send for LateInit<T> {}
unsafe impl<T : Sync + Send + Clone> Sync for LateInit<T> {}
