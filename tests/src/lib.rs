use async_trait::async_trait;
use integration_trait::make_integration_version;
use near_sdk::{json_types::U128, PromiseOrValue};

#[make_integration_version]
pub trait ContractNameInterface {
    fn init() -> Self;
    fn initialize_with_name(name: String) -> Self;

    fn receive_name(&self) -> String;
    fn set_name(&mut self, name: String);

    fn burn(&mut self) -> PromiseOrValue<U128>;
}

impl ContractNameInterface for () {
    fn init() -> Self {}

    fn initialize_with_name(_name: String) -> Self {}

    fn receive_name(&self) -> String {
        Default::default()
    }

    fn set_name(&mut self, _name: String) {}

    fn burn(&mut self) -> PromiseOrValue<U128> {
        todo!()
    }
}

#[async_trait]
impl ContractNameInterfaceIntegration for () {
    async fn init(&self) -> anyhow::Result<()> {
        Ok(())
    }

    async fn initialize_with_name(&self, _name: String) -> anyhow::Result<()> {
        Ok(())
    }

    async fn receive_name(&self) -> anyhow::Result<String> {
        Ok(Default::default())
    }

    async fn set_name(&mut self, _name: String) -> anyhow::Result<()> {
        Ok(())
    }

    async fn burn(&mut self) -> anyhow::Result<U128> {
        todo!()
    }
}
