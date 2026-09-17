#[async_trait]
pub trait Policy {
    async fn acquire(&self, type_: ResourceType) -> anyhow::Result<ResourcePermit>;
}

#[derive(Clone)]
pub struct ResourcePermit(Arc<dyn Any>);

#[derive(Clone)]
pub struct ResourceSemaphore(Arc<dyn Policy>);

impl ResourceSemaphore {
    pub fn builder(policy: impl Policy) -> ResourceSemaphoreBuilder {
        ResourceSemaphoreBuilder(Arc::new(policy))
    }

    pub async fn acquire(&self, type_: ResourceType) -> anyhow::Result<ResourcePermit> {
        self.0.acquire(type_).await
    }

    pub fn bounded(&self) -> (Self, ResourceBound) {
        let (policy, bound) = self.0.bounded();
        (Self(policy), bound)
    }

    pub fn attribute(&self, permit: &ResourcePermit) {
        self.0.attribute(permit);
    }
}

pub struct ResourceSemaphoreBuilder(Arc<dyn Policy>);

impl ResourceSemaphoreBuilder {
    pub fn build(self) -> ResourceSemaphore {
        ResourceSemaphore(self.0)
    }

    pub fn with_connection_semaphore(self, semaphore: ConnectionSemaphore) -> Self {
        Self(Arc::new(WithConnectionSemaphore {
            semaphore,
            next: self.0,
        }))
    }
}

struct WithConnectionSemaphore {
    semaphore: ConnectionSemaphore,
    next: Arc<dyn Policy>,
}

#[async_trait]
impl Policy for WithConnectionSemaphore {
    async fn acquire(&self, type_: ResourceType) -> anyhow::Result<ResourcePermit> {
        ResourcePermit(Arc::new((
            self.semaphore.acquire()?,
            self.next.acquire(type_)?,
        )))
    }
}
