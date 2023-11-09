use crate::domain::Env;

/// This trait is to be implemented when you add a new Command or Subcommand, the purpose of this
/// trait is to be able to share the global Configuration from the CLI to every sub-commands.
#[async_trait::async_trait]
pub trait SwarmdCommand {
    type Error;

    async fn execute(&self, env: &Env) -> Result<(), Self::Error>;
}
