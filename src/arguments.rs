use super::*;

#[derive(clap::Parser)]
pub(crate) struct Arguments {
  #[command(subcommand)]
  pub(crate) subcommand: Subcommand,
}
