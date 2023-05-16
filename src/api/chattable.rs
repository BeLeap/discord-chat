use async_trait::async_trait;

#[async_trait]
pub trait Chattable {
    type Error;

    async fn chat(self: &Self, instruction: String) -> Result<String, Self::Error>;
}
