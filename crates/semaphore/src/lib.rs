use anyhow::anyhow;
use async_trait::async_trait;
use spin_connection_semaphore::{ConnectionPermit, ConnectionSemaphore};
use std::any::Any;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::sync::{Arc, Mutex};

#[derive(Clone, Copy)]
pub enum Type {
    FileDescriptor,
    Socket,
}

pub trait PermitInner: Any + Send + Sync {
    fn set_active(&self, active: bool);
}

#[derive(Clone)]
pub struct Permit(Arc<dyn PermitInner>);

impl Permit {
    fn set_active(&self, active: bool) {
        self.0.set_active(active);
    }
}

impl Hash for Permit {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.0).hash(state);
    }
}

impl PartialEq for Permit {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for Permit {}

#[async_trait]
pub trait SemaphoreInner: Any + Send + Sync {
    async fn acquire(&self, type_: Type) -> anyhow::Result<Permit>;
}

#[derive(Clone)]
pub struct Semaphore(Arc<dyn SemaphoreInner>);

impl Semaphore {
    pub fn unlimited() -> SemaphoreBuilder {
        Self::builder(UnlimitedSemaphore)
    }

    pub fn builder(inner: impl SemaphoreInner) -> SemaphoreBuilder {
        SemaphoreBuilder(Arc::new(inner))
    }

    pub fn as_builder(&self) -> SemaphoreBuilder {
        SemaphoreBuilder(self.0.clone())
    }

    pub async fn acquire(&self, type_: Type) -> anyhow::Result<Permit> {
        self.0.acquire(type_).await
    }
}

pub struct SemaphoreBuilder(Arc<dyn SemaphoreInner>);

impl SemaphoreBuilder {
    pub fn build(&self) -> Semaphore {
        Semaphore(self.0.clone())
    }

    pub fn with_connection_semaphore(&self, connection_semaphore: ConnectionSemaphore) -> Self {
        Self(Arc::new(WithConnectionSemaphore {
            inner: self.0.clone(),
            connection_semaphore,
        }))
    }
}

struct WithConnectionSemaphore {
    inner: Arc<dyn SemaphoreInner>,
    connection_semaphore: ConnectionSemaphore,
}

#[async_trait]
impl SemaphoreInner for WithConnectionSemaphore {
    async fn acquire(&self, type_: Type) -> anyhow::Result<Permit> {
        Ok(Permit(Arc::new(WithConnectionPermit {
            inner: self.inner.acquire(type_).await?,
            _connection_permit: match type_ {
                Type::FileDescriptor => None,
                Type::Socket => Some(
                    self.connection_semaphore
                        .try_acquire()
                        .ok_or_else(|| anyhow!("connection limit reached"))?,
                ),
            },
        })))
    }
}

struct WithConnectionPermit {
    inner: Permit,
    _connection_permit: Option<ConnectionPermit>,
}

impl PermitInner for WithConnectionPermit {
    fn set_active(&self, active: bool) {
        self.inner.set_active(active);
    }
}

#[derive(Clone, Default)]
pub struct ActivityGuard(Arc<Mutex<HashSet<Permit>>>);

impl ActivityGuard {
    pub fn add(&self, permit: Permit) {
        permit.set_active(true);
        self.0.try_lock().unwrap().insert(permit);
    }
}

impl Drop for ActivityGuard {
    fn drop(&mut self) {
        for permit in self.0.try_lock().unwrap().deref() {
            permit.set_active(false);
        }
    }
}

struct UnlimitedSemaphore;

#[async_trait]
impl SemaphoreInner for UnlimitedSemaphore {
    async fn acquire(&self, _type_: Type) -> anyhow::Result<Permit> {
        Ok(Permit(Arc::new(UnlimitedPermit)))
    }
}

struct UnlimitedPermit;

#[async_trait]
impl PermitInner for UnlimitedPermit {
    fn set_active(&self, _active: bool) {
        // ignore
    }
}
