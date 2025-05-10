use tracing::instrument;
use which_shell::ShellVersion;

#[instrument]
pub async  fn which_shell() -> Option<ShellVersion> {
    // which_shell::which_shell()
    None
}
