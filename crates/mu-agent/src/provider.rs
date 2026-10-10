use crate::{Context, Model};

pub trait Provider {
    //this will be stream() fn later
    fn complete(
        &self,
        model: &Model,
        context: &Context,
    ) -> impl Future<Output = Result<String, String>> + Send;
}
