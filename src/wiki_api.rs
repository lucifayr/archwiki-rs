#[derive(Debug, Clone, serde::Deserialize)]
pub struct ApiResponse<T> {
    pub query: T,
}
