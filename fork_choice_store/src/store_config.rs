use core::ops::Mul as _;

use educe::Educe;
use types::config::Config as ChainConfig;

#[derive(Clone, Copy, Educe)]
#[educe(Default)]
pub struct StoreConfig {
    #[educe(Default = 32)]
    pub max_empty_slots: u64,
    #[educe(Default = 128)]
    pub unfinalized_states_in_memory: u64,
}

impl StoreConfig {
    /// Returns a configuration more likely to trigger bugs.
    ///
    /// Intended for use in tests.
    #[must_use]
    pub fn aggressive(chain_config: &ChainConfig) -> Self {
        Self {
            unfinalized_states_in_memory: Self::min_unfinalized_states_in_memory(chain_config),
            ..Self::default()
        }
    }

    #[must_use]
    pub fn min_unfinalized_states_in_memory(chain_config: &ChainConfig) -> u64 {
        // The minimum was chosen arbitrarily to protect users from bugs based on the intuition
        // that blocks older than 2 epochs should rarely be needed in well-behaved networks.
        // We did find a bug in `ChainLink::unload_old_states` while implementing in-memory mode.
        chain_config
            .preset_base
            .phase0_preset()
            .slots_per_epoch()
            .get()
            .mul(2)
    }
}
