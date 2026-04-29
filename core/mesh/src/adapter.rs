use async_trait::async_trait;
use crate::error::Result;

//& Plarform--agonistic BLE mesh adapter contract
#[async_trait]
pub trait MeshAdapter: Send + Sync {
    //? Initialize adapter resources
    async fn start(&self) -> Result<()>;

    //? Advertise bytes to nearby peers
    async fn advertise(&self, data: Vec<u8>) -> Result<()>;

    //? Scan for nearby peers
    async fn scan(&self) -> Result<()>;

    //? Sendraw bytes into mesh transport
    async fn send(&self, data: Vec<u8>) -> Result<()>;

    //? Receive a raw packet if available
    async fn receive(&self) -> Result<Option<Vec<u8>>>;
}
