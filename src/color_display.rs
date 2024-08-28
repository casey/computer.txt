use super::*;

pub(crate) trait ColorDisplay {
  fn color_display(&self, color: bool) -> Wrapper
  where
    Self: Sized,
  {
    Wrapper(self, color)
  }

  fn fmt(&self, f: &mut Formatter, color: bool) -> fmt::Result;
}

pub(crate) struct Wrapper<'a>(&'a dyn ColorDisplay, bool);

impl<'a> Display for Wrapper<'a> {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    self.0.fmt(f, self.1)
  }
}
