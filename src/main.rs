use {
  self::{
    arguments::Arguments, color_display::ColorDisplay, computer::Computer, lexer::Lexer,
    subcommand::Subcommand, token::Token,
  },
  clap::Parser,
  owo_colors::OwoColorize,
  std::{
    error::Error,
    fmt::{self, Display, Formatter},
    fs,
    io::{self, IsTerminal},
    path::{Path, PathBuf},
    str::{Chars, FromStr},
  },
};

const ACC: usize = 0x0;
const PC: usize = 0xC;
const REGISTERS: usize = 16;
const MEMORY: usize = 256;

mod arguments;
mod color_display;
mod computer;
mod lexer;
mod subcommand;
mod token;

type Result<T = (), E = Box<dyn Error>> = std::result::Result<T, E>;

fn main() -> Result {
  let arguments = Arguments::parse();

  arguments.subcommand.run()
}

#[cfg(test)]
mod tests {
  use {
    super::*,
    comrak::{nodes::NodeValue, parse_document, Arena, Options},
    pretty_assertions::assert_eq,
  };

  #[test]
  fn readme() {
    let arena = Arena::new();

    let root = parse_document(
      &arena,
      &fs::read_to_string("README.md").unwrap(),
      &Options::default(),
    );

    let mut examples = Vec::new();

    for node in root.descendants() {
      if let NodeValue::CodeBlock(code_block) = &node.data.borrow().value {
        if code_block.info == "computer" {
          examples.push(code_block.literal.clone());
        }
      }
    }

    for example in examples.chunks(2) {
      if example.len() != 2 {
        panic!("odd number of computer code blocks in readme");
      }
      let mut computer = example[0].parse::<Computer>().unwrap();
      computer.run();
      assert_eq!(
        computer.color_display(false).to_string(),
        example[1].strip_suffix('\n').unwrap(),
      );
    }
  }
}
