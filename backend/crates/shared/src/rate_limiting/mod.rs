use actix_governor::{
    Governor, GovernorConfig, GovernorConfigBuilder, PeerIpKeyExtractor,
    governor::{clock::QuantaInstant, middleware::NoOpMiddleware},
};

use crate::rate_limiting::config::RateLimitingConfig;

pub mod config;

pub fn create_governor_config(
    config: RateLimitingConfig,
) -> Option<GovernorConfig<PeerIpKeyExtractor, NoOpMiddleware<QuantaInstant>>> {
    GovernorConfigBuilder::default()
        .requests_per_second(config.requests_per_second)
        .burst_size(config.burst_size)
        .finish()
}

pub fn create_rate_limiting(
    governor_config: &GovernorConfig<PeerIpKeyExtractor, NoOpMiddleware<QuantaInstant>>,
) -> Governor<PeerIpKeyExtractor, NoOpMiddleware<QuantaInstant>> {
    Governor::new(governor_config)
}
