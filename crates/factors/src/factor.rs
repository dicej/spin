use std::any::Any;

use anyhow::Context;

use crate::{
    prepare::FactorInstanceBuilder, runtime_config::RuntimeConfigTracker, App, FactorRuntimeConfig,
    InstanceBuilders, Linker, PrepareContext, RuntimeConfigSource, RuntimeFactors,
};

pub trait Factor: Any + Sized {
    type RuntimeConfig: FactorRuntimeConfig;

    type AppState;

    type InstanceBuilder: FactorInstanceBuilder;

    /// Initializes this Factor for a runtime. This will be called at most once,
    /// before any call to [`FactorInstanceBuilder::new`]
    fn init<T: RuntimeFactors>(&mut self, mut ctx: InitContext<T, Self>) -> anyhow::Result<()> {
        _ = &mut ctx;
        Ok(())
    }

    /// Performs factor-specific validation and configuration for the given
    /// [`App`].
    ///
    /// A runtime may - but is not required to - reuse the returned config
    /// across multiple instances. Note that this may be called without any call
    /// to `init` in cases where only validation is needed.
    fn configure_app<T: RuntimeFactors>(
        &self,
        ctx: ConfigureAppContext<T, Self>,
    ) -> anyhow::Result<Self::AppState>;

    /// Prepares an instance builder for this factor.
    ///
    /// This method is given access to the app component being instantiated and
    /// to any other factors' instance builders that have already been prepared.
    fn prepare<T: RuntimeFactors>(
        &self,
        ctx: PrepareContext<Self>,
        _builders: &mut InstanceBuilders<T>,
    ) -> anyhow::Result<Self::InstanceBuilder>;
}

/// The instance state of the given [`Factor`] `F`.
pub type FactorInstanceState<F> =
    <<F as Factor>::InstanceBuilder as FactorInstanceBuilder>::InstanceState;

pub(crate) type GetDataFn<Facts, F> =
    fn(&mut <Facts as RuntimeFactors>::InstanceState) -> &mut FactorInstanceState<F>;

/// An InitContext is passed to [`Factor::init`], giving access to the global
/// common [`wasmtime::component::Linker`].
pub struct InitContext<'a, T: RuntimeFactors, F: Factor> {
    pub(crate) linker: &'a mut Linker<T>,
    pub(crate) get_data: GetDataFn<T, F>,
}

impl<'a, T: RuntimeFactors, F: Factor> InitContext<'a, T, F> {
    #[doc(hidden)]
    pub fn new(linker: &'a mut Linker<T>, get_data: GetDataFn<T, F>) -> Self {
        Self { linker, get_data }
    }

    pub fn linker(&mut self) -> &mut Linker<T> {
        self.linker
    }

    pub fn get_data_fn(&self) -> GetDataFn<T, F> {
        self.get_data
    }

    pub fn link_bindings(
        &mut self,
        add_to_linker: impl Fn(
            &mut Linker<T>,
            fn(&mut T::InstanceState) -> &mut FactorInstanceState<F>,
        ) -> anyhow::Result<()>,
    ) -> anyhow::Result<()> {
        add_to_linker(self.linker, self.get_data)
    }
}

pub struct ConfigureAppContext<'a, T: RuntimeFactors, F: Factor> {
    app: &'a App,
    app_state: &'a T::AppState,
    runtime_config: Option<F::RuntimeConfig>,
}

impl<'a, T: RuntimeFactors, F: Factor> ConfigureAppContext<'a, T, F> {
    #[doc(hidden)]
    pub fn new<S: RuntimeConfigSource>(
        app: &'a App,
        app_state: &'a T::AppState,
        runtime_config_tracker: &mut RuntimeConfigTracker<S>,
    ) -> anyhow::Result<Self> {
        let runtime_config = runtime_config_tracker.get_config::<F>()?;
        Ok(Self {
            app,
            app_state,
            runtime_config,
        })
    }

    pub fn app(&self) -> &App {
        self.app
    }

    pub fn app_state<U: Factor>(&self) -> crate::Result<&U::AppState> {
        T::app_state::<U>(self.app_state).context("no such factor")
    }

    pub fn runtime_config(&self) -> Option<&F::RuntimeConfig> {
        self.runtime_config.as_ref()
    }

    pub fn take_runtime_config(&mut self) -> Option<F::RuntimeConfig> {
        self.runtime_config.take()
    }
}

pub struct ConfiguredApp<T: RuntimeFactors> {
    app: App,
    app_state: T::AppState,
}

impl<T: RuntimeFactors> ConfiguredApp<T> {
    #[doc(hidden)]
    pub fn new(app: App, app_state: T::AppState) -> Self {
        Self { app, app_state }
    }

    pub fn app(&self) -> &App {
        &self.app
    }

    pub fn app_state<F: Factor>(&self) -> crate::Result<&F::AppState> {
        T::app_state::<F>(&self.app_state).context("no such factor")
    }
}
