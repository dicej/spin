//! Commands for the Spin CLI.

/// Commands for building Spin applications.
pub mod build;
/// Commands for publishing applications to the Fermyon Platform.
pub mod cloud;
/// Hidden command for instantiating all the apps in a repository.
pub mod crater;
/// Command for running the Spin Doctor.
pub mod doctor;
/// Commands for external subcommands (i.e. plugins)
pub mod external;
/// Command for creating a new application.
pub mod new;
/// Command for adding a plugin to Spin
pub mod plugins;
/// Commands for working with OCI registries.
pub mod registry;
/// Commands for working with templates.
pub mod templates;
/// Commands for starting the runtime.
pub mod up;
/// Command for rebuilding and restarting a Spin app when files change.
pub mod watch;
