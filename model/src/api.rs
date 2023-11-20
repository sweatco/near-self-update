use integration_trait::make_integration_version;
use near_sdk::{AccountId, PromiseOrValue};
use near_self_update_model::VersionMetadata;

#[make_integration_version]
pub trait ContractApi {
    fn init(manager: AccountId) -> Self;
    fn version_metadata(&self) -> VersionMetadata;
    #[update]
    fn update_contract(&mut self) -> PromiseOrValue<()>;
    fn after_update(&mut self);
}
