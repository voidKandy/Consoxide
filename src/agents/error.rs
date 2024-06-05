use crate::{errors::error_chain_fmt, language_models::completions::error::CompletionError};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use super::listeners;

pub type AgentResult<T> = Result<T, AgentError>;
#[derive(thiserror::Error)]
pub enum AgentError {
    #[error(transparent)]
    Undefined(#[from] anyhow::Error),
    CompletionError(#[from] CompletionError),
    Listener(#[from] listeners::error::ListenerError),
}

impl Debug for AgentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        error_chain_fmt(self, f)
    }
}

impl Display for AgentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let display = match self {
            Self::Listener(err) => err.to_string(),
            Self::Undefined(err) => err.to_string(),
            Self::CompletionError(err) => err.to_string(),
        };
        write!(f, "{}", display)
    }
}
