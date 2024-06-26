use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::domain::models::session_token::SessionToken;

pub struct DeleteSessionSelf {
    pub id_provider: Box<dyn IdProvider>
}

impl Interactor<Option<SessionToken>, SessionToken> for DeleteSessionSelf {
    async fn execute(&self, data: Option<SessionToken>) -> Result<SessionToken, ApplicationError> {
        
        if !self.id_provider.is_auth() {
            return Err(ApplicationError::Unauthorized(
                ErrorContent::Message("Unauthorized".to_string())
            ));
        }
        
        match data {
            None => {
                return Err(ApplicationError::Unauthorized(
                    ErrorContent::Message(
                        "Your method of authorization does not allow you to perform this operation"
                            .to_string()
                    )
                ));
            },
            Some(_) => {}
        }
        
        Ok(data.unwrap())
    }
}
