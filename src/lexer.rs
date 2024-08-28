use super::*;

pub(crate) struct Lexer<'src> {
  chars: Chars<'src>,
  next: Option<char>,
  column: usize,
  line: usize,
}

impl<'src> Lexer<'src> {
  pub(crate) fn lex(src: &'src str) -> Result<Vec<Token>> {
    let mut chars = src.chars();
    let next = chars.next();
    Self {
      chars,
      column: 0,
      line: 0,
      next,
    }
    .tokenize()
  }

  fn advance(&mut self) -> Result {
    match self.next {
      Some(c) => {
        if c == '\n' {
          self.line += 1;
          self.column = 0;
        } else {
          self.column += 1;
        }

        self.next = self.chars.next();
        Ok(())
      }
      None => Err(format!("lexer advanced past end of text").into()),
    }
  }

  fn tokenize(mut self) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();

    while let Some(next) = self.next {
      match next {
        'R' => {
          self.advance()?;
          tokens.push(Token::Register(
            self.next.unwrap().to_digit(16).unwrap() as u8
          ));
          self.advance()?;
          assert_eq!(self.next.unwrap(), ':');
          self.advance()?;
        }
        '0'..='9' | 'a'..='f' | 'A'..='F' => {
          let mut byte = String::new();
          byte.push(next);
          self.advance()?;
          byte.push(self.next.unwrap());
          let byte = u8::from_str_radix(&byte, 16)?;
          self.advance()?;
          tokens.push(if self.next == Some(':') {
            self.advance()?;
            Token::Memory(byte)
          } else {
            Token::Byte(byte)
          })
        }
        '#' => {
          while self.next != Some('\n') {
            self.advance()?;
          }
          self.advance()?;
        }
        ' ' | '\n' | '\r' => {
          self.advance()?;
        }
        '\u{1b}' => {
          self.advance()?;
          assert_eq!(self.next.unwrap(), '[');

          loop {
            self.advance()?;

            let next = self.next.unwrap() as u32;

            if next >= 0x40 && next <= 0x7E {
              self.advance()?;
              break;
            }
          }
        }
        _ => {
          return Err(
            format!(
              "{}:{} unexpected character: {next:?}",
              self.line + 1,
              self.column + 1
            )
            .into(),
          );
        }
      }
    }

    Ok(tokens)
  }
}
