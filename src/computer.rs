use super::*;

#[derive(Debug, PartialEq)]
pub(crate) struct Computer {
  pub(crate) registers: Box<[u8]>,
  pub(crate) memory: Box<[u8]>,
}

impl Computer {
  pub(crate) fn run(&mut self) {
    while self.memory[self.registers[PC] as usize] != 0xFF {
      self.step();
    }
  }

  pub(crate) fn step(&mut self) {
    let pc = self.registers[PC] as usize;

    let instruction = self.memory[pc];
    let operand = self.memory[(pc + 1) % MEMORY];

    match instruction {
      0x10 => {
        self.registers[ACC] = self.registers[ACC].wrapping_add(self.memory[operand as usize]);
        self.registers[PC] += 2;
      }
      0x20 => {
        self.memory[operand as usize] = self.registers[ACC];
        self.registers[PC] += 2;
      }
      0xFF => {}
      _ => todo!(),
    }
  }
}

impl Default for Computer {
  fn default() -> Self {
    Self {
      registers: vec![0; REGISTERS].into(),
      memory: vec![0; MEMORY].into(),
    }
  }
}

impl ColorDisplay for Computer {
  fn fmt(&self, f: &mut Formatter, color: bool) -> fmt::Result {
    let mut printed = 0;
    for (i, row) in self.registers.chunks(4).enumerate() {
      let last = match row
        .iter()
        .enumerate()
        .rev()
        .find(|(_j, register)| **register != 0)
      {
        Some((j, _)) => j + 1,
        None => continue,
      };

      if printed > 0 {
        writeln!(f)?;
      }

      for (j, register) in row[0..last].iter().enumerate() {
        if j > 0 {
          write!(f, " ")?;
        }

        write!(f, "R{:X}: ", i * 4 + j)?;
        write!(f, "{register:02x}")?;
      }

      printed += 1;
    }

    if printed > 0 {
      writeln!(f)?;
    }

    writeln!(f)?;

    let mut printed = 0;
    for (i, row) in self.memory.chunks(16).enumerate() {
      let last = match row
        .iter()
        .enumerate()
        .rev()
        .find(|(_j, value)| **value != 0)
      {
        Some((j, _)) => j + 1,
        None => continue,
      };

      if printed > 0 {
        writeln!(f)?;
      }

      let base = i * 16;

      write!(f, "{base:02X}: ")?;

      for (j, byte) in row[0..last].iter().enumerate() {
        if j > 0 {
          write!(f, " ")?;
        }

        if base + j == self.registers[PC].into() && color {
          write!(f, "{:02X}", byte.bright_cyan())?;
        } else {
          write!(f, "{byte:02X}")?;
        }
      }

      printed += 1;
    }

    Ok(())
  }
}

impl FromStr for Computer {
  type Err = Box<dyn Error>;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let tokens = Lexer::lex(s)?;

    let mut computer = Self::default();

    let mut registers = true;
    let mut pointer = 0;

    for token in tokens {
      match token {
        Token::Byte(b) => {
          if registers {
            computer.registers[pointer] = b;
          } else {
            computer.memory[pointer] = b;
          }
          pointer += 1;

          if registers && pointer == REGISTERS {
            registers = false;
            pointer = 0;
          }
        }
        Token::Memory(p) => {
          registers = false;
          pointer = usize::from(p);
        }
        Token::Register(p) => {
          registers = true;
          pointer = usize::from(p);
        }
      }
    }

    Ok(computer)
  }
}
