//! # Configuration
//! The `config` module provides the `ActorSystemConfig` struct, which is used
//! to configure the properties of an `ActorSystem` during its creation.

use serde::{Deserialize, Serialize};
use std::time::Duration;

pub const NO_HOST: &str = "nohost";
pub const CLIENT: &str = "$client";

/// The `ActorSystemConfig` struct allows you to set various configuration parameters
/// such as the name of the actor system and the number of worker threads for the
/// underlying runtime. You can create a new `ActorSystemConfig`, set its properties,
/// and pass it to the `ActorSystem::new` method to create a new `ActorSystem` with the
/// specified configuration.
///
/// # Example
///
/// ```
/// use protoactor::config::{ActorSystemConfig, ActorSystemConfigBuilder};
/// use protoactor::actor_system::ActorSystem;
///
/// // using builder pattern
/// let config = ActorSystemConfigBuilder::new()
///     .with_name("my_actor_system")
///     .with_worker_threads(4)
///     .build();
///
/// let system = ActorSystem::new(config);
/// // Perform operations with the actor system.
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActorSystemConfig {
    /// The name of the actor system.
    pub name: String,

    /// The number of worker threads for the underlying runtime.
    /// Defaults to the number of logical cores.
    pub worker_threads: usize,

    /// The host name or IP address of the actor system. Defaults to "nohost" if not specified.
    pub host: String,

    /// The tcp port of the actor system to listen on. Defaults to 0 if not specified and the actor
    /// system will use a random available port when start listening for incoming connections.
    pub port: u16,

    /// The interval used to trigger throttling of deadletter message logs.
    pub dead_letter_throttle_interval: Duration,

    /// The counter used to trigger throttling of deadletter message logs.
    /// DeadLetter throttling triggers when there are `dead_letter_throttle_count` deadletters in
    /// `dead_letter_throttle_interval` time frame.
    pub dead_letter_throttle_count: usize,

    /// Enables logging for DeadLetter events in request/request_async (when a message reaches
    /// DeadLetter instead of target actor)
    ///
    /// When set to `false`, the requesting code is responsible for logging it manually.
    /// Default: `true`
    pub dead_letter_request_logging: bool,

    /// Developer debugging feature, enables logging for actor supervision events.
    /// Default: `false`
    pub developer_supervision_logging: bool,

    /// `metrics_enabled` enables the collection of metrics for the actor system. Set to `true`
    /// if you want to export the metrics with opentelemetry exporters.
    /// Default: `false`
    pub metrics_enabled: bool,
    /*/// `configure_root_context` is a callback that allows you to configure the actor system
    /// root context.
    /// Default: |context| context
    pub configure_root_context: fn(context: &mut crate::actor::Context) -> &mut crate::actor::Context,*/
}

impl ActorSystemConfig {
    /// Creates a new `ActorSystemConfig` struct.
    pub fn builder() -> ActorSystemConfigBuilder {
        ActorSystemConfigBuilder::new()
    }
}

impl Default for ActorSystemConfig {
    fn default() -> Self {
        // detect number of logical cores
        let num_cpus = num_cpus::get();
        ActorSystemConfig {
            name: "local".to_string(),
            worker_threads: num_cpus,
            host: NO_HOST.to_string(),
            port: 0,
            dead_letter_throttle_interval: Duration::from_secs(1),
            dead_letter_throttle_count: 10,
            dead_letter_request_logging: true,
            developer_supervision_logging: false,
            metrics_enabled: false,
            /*configure_root_context: |context| context,*/
        }
    }
}

/// The `ActorSystemConfigBuilder` struct is used to build an `ActorSystemConfig` struct.
#[derive(Default)]
pub struct ActorSystemConfigBuilder {
    config: ActorSystemConfig,
}

impl ActorSystemConfigBuilder {
    /// Creates a new `ActorSystemConfigBuilder` struct.
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the name of the actor system.
    pub fn with_name(mut self, name: &str) -> Self {
        self.config.name = name.to_string();
        self
    }

    /// Sets the number of worker threads for the underlying runtime.
    pub fn with_worker_threads(mut self, worker_threads: usize) -> Self {
        self.config.worker_threads = worker_threads;
        self
    }

    /// Sets the host name or IP address of the actor system.
    pub fn with_host(mut self, host: &str) -> Self {
        self.config.host = host.to_string();
        self
    }

    /// Sets the tcp port of the actor system to listen on.
    pub fn with_port(mut self, port: u16) -> Self {
        self.config.port = port;
        self
    }

    /// Builds the `ActorSystemConfig` struct.
    pub fn build(self) -> ActorSystemConfig {
        self.config
    }
}
