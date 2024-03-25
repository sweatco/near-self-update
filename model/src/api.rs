#[cfg(feature = "release-api")]
use near_sdk::AccountId;
#[cfg(feature = "integration-api")]
use nitka::AccountId;
use nitka_proc::make_integration_version;

#[cfg(feature = "integration-test")]
pub struct UpdateContract<'a> {
    pub contract: &'a near_workspaces::Contract,
}

#[make_integration_version]
pub trait ContractApi {
    fn init(manager: AccountId) -> Self;
}
