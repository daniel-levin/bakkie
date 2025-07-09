pub mod prompts;
pub mod resources;
pub mod tools;

use self::tools::Tools;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Default)]
pub struct Provisions {
    tools: Arc<RwLock<Tools>>,
}
