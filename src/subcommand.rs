use super::*;

#[derive(clap::Subcommand)]
pub(crate) enum Subcommand {
  Init,
  Load { computer: PathBuf },
  Run { computer: PathBuf },
  Step { computer: PathBuf },
}

impl Subcommand {
  pub(crate) fn run(self) -> Result {
    match self {
      Self::Init => Self::print(Computer::default()),
      Self::Load { computer } => {
        Self::print(Self::load(&computer)?);
      }
      Self::Run { computer } => {
        let mut computer = Self::load(&computer)?;
        computer.run();
        Self::print(computer);
      }
      Self::Step { computer } => {
        let mut computer = Self::load(&computer)?;
        computer.step();
        Self::print(computer);
      }
    }

    Ok(())
  }

  fn load(path: &Path) -> Result<Computer> {
    Ok(fs::read_to_string(path)?.parse()?)
  }

  fn print(computer: Computer) {
    println!("{}", computer.color_display(io::stdout().is_terminal()));
  }
}
