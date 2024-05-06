use crate::state_transition::state_transitions::identity::identity_update_transition::IdentityUpdateTransition;
use crate::state_transition::FeatureVersioned;
use platform_version::version::FeatureVersion;

impl FeatureVersioned for IdentityUpdateTransition {
    fn feature_version(&self) -> FeatureVersion {
        match self {
            IdentityUpdateTransition::V0(v0) => v0.feature_version(),
        }
    }
}
