use mu_agent::{AssistantMessage, Context, Model, provider::Provider};

pub const PROVIDER_ID: &str = "fake";
pub const MODEL_ID: &str = "fake";

pub fn model() -> Model {
    Model {
        id: MODEL_ID.to_owned(),
    }
}

pub struct FakeProvider;

impl Provider for FakeProvider {
    async fn complete(
        &self,
        _model: &Model,
        _context: &Context,
    ) -> Result<AssistantMessage, String> {
        Ok(AssistantMessage {
            content: "FakeProvider answer".to_owned(),
        })
    }
}
