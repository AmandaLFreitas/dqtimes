use thiserror::Error;

#[derive(Debug, Error)]
pub enum ForecastError {
    #[error("dados de entrada inválidos: {0}")]
    InvalidInput(String),

    #[error("configuração inválida: {0}")]
    ConfigError(String),

    #[error("algoritmo não implementado")]
    NotImplemented,

    #[error("falha durante o cálculo: {0}")]
    ComputationError(String),
}
