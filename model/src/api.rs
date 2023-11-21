use integration_trait::make_integration_version;
use near_sdk::AccountId;

#[make_integration_version]
pub trait ContractApi {
    fn init(manager: AccountId) -> Self;
}
