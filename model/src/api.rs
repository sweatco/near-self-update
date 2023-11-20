use integration_trait::make_integration_version;
use near_self_update_model::VersionMetadata;

#[make_integration_version]
pub trait ContractApi {
    fn init() -> Self;
    fn version_metadata(&self) -> VersionMetadata;
}
