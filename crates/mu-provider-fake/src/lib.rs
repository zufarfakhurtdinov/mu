use mu_agent::{Context, Model, provider::Provider};

pub fn model() -> Model {
    Model {
        id: "fake".to_owned(),
    }
}

pub struct FakeProvider;

impl Provider for FakeProvider {
    async fn complete(&self, _model: &Model, _context: &Context) -> Result<String, String> {
        Ok("FakeProvider answer".to_owned())
    }
}
