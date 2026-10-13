use mu_agent::{Context, Model, provider::Provider};
use mu_provider_fake::FakeProvider;

pub enum ModelProvider {
    Fake(FakeProvider),
}

impl ModelProvider {
    pub fn from_id(id: &str) -> Result<Self, String> {
        match id {
            mu_provider_fake::PROVIDER_ID => Ok(Self::Fake(FakeProvider)),
            _ => Err(format!("unknown provider '{id}'")),
        }
    }

    pub fn models(&self) -> Vec<Model> {
        match self {
            Self::Fake(_) => vec![mu_provider_fake::model()],
        }
    }

    pub fn find_model(&self, model_id: &str) -> Option<Model> {
        self.models().into_iter().find(|model| model.id == model_id)
    }
}

impl Provider for ModelProvider {
    async fn complete(&self, model: &Model, context: &Context) -> Result<String, String> {
        match self {
            Self::Fake(provider) => provider.complete(model, context).await,
        }
    }
}
