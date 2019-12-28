use std::collections::HashMap;

/// Reserved long flag name for help
pub const HELP_LONG: &str = "help";
/// Reserved short flag name for help
pub const HELP_SHORT: &str = "h";

/// Trait for types that can be converted from a flag.
/// The parse_from function was made specific due to the dynamic nature of the flag set.
/// But if there is some way to make it more generic that would be appreciated.
pub trait Flaggable {
  /// Whether or not this flag expects a value.
  /// In the case this is overriden, the parse_from method should ignore the input value.
  fn expects_value(&self) -> bool { true }

  /// Parses a string into this flag.
  /// The string is value subsequently after the flag
  fn parse_from(&mut self, s: &str) -> Result<(), String>;
}

/// Implements flaggable for Option types that wrap things that can be parsed.
impl<T> Flaggable for Option<T>
where
  T: std::str::FromStr,
{
  fn parse_from(&mut self, s: &str) -> Result<(), String> {
    match T::from_str(s) {
      Err(_) => Err(format!("Failed to parse {}", s)),
      Ok(v) => {
        self.replace(v);
        Ok(())
      },
    }
  }
}

/// Implements a togglable bool
/// If the flag is passed, it toggles the input value.
impl Flaggable for bool {
  fn expects_value(&self) -> bool { false }

  fn parse_from(&mut self, _: &str) -> Result<(), String> {
    *self = !*self;
    Ok(())
  }
}

#[derive(Default)]
pub struct FlagSet<'a> {
  mappings: HashMap<&'static str, &'a mut dyn Flaggable>,
  help_info: HashMap<&'static str, &'static str>,
}

fn show_help(h: &HashMap<&str, &str>) {
  h.iter().for_each(|(flag, info)| {
    eprintln!("--{}", flag);
    eprintln!("\t {}", info);
  });
}

impl<'a> FlagSet<'a> {
  /// Creates a new empty FlagSet
  pub fn new() -> Self {
    Self {
      mappings: HashMap::new(),
      help_info: HashMap::new(),
    }
  }
  /// Adds something flaggable with a given name and help message to the flag set.
  /// Panics if the name is one of the reserved help flags(help or h).
  pub fn add<F: Flaggable>(&mut self, name: &'static str, help: &'static str, f: &'a mut F) {
    if name == HELP_LONG || name == HELP_SHORT {
      panic!("Cannot override help flag");
    }
    self.mappings.insert(name, f);
    self.help_info.insert(name, help);
  }
  /// Parses an iterator of strings into this flag set.
  /// Returns unused values
  // TODO decide whether this should consume the flag set or not
  pub fn parse<I>(&mut self, mut i: I) -> Result<Vec<String>, ParseError>
  where
    I: Iterator<Item = String>, {
    let mut out = vec![];
    while let Some(v) = i.next() {
      if !v.starts_with('-') {
        out.push(v);
        continue;
      }
      let v = v.trim_start_matches('-');
      if v == HELP_LONG || v == HELP_SHORT {
        return Err(ParseError::HelpRequested);
      }
      match self.mappings.get_mut(&*v) {
        None => return Err(ParseError::UnknownFlag(v.to_string())),
        Some(ref mut flag) => {
          if !flag.expects_value() {
            flag
              .parse_from("")
              .map_err(ParseError::ParseFromFailure)?;
            continue;
          }
          let flag_val = match i.next() {
            None => return Err(ParseError::MissingValue(v.to_string())),
            Some(flag_val) => flag_val,
          };
          flag
            .parse_from(&flag_val)
            .map_err(ParseError::ParseFromFailure)?;
        },
      };
    }
    Ok(out)
  }
  pub fn parse_args(&mut self) -> Vec<String> {
    use std::env::args;
    match self.parse(args().skip(1)) {
      Ok(rem) => rem,
      Err(e) => {
        let status = match e {
          ParseError::HelpRequested => 0,
          ParseError::ParseFromFailure(v) => {
            eprintln!("{}", v);
            1
          },
          ParseError::UnknownFlag(f) => {
            eprintln!("Unknown flag -{}", f);
            1
          },
          ParseError::MissingValue(f) => {
            eprintln!("Missing value for flag -{}", f);
            1
          },
        };
        show_help(&self.help_info);
        std::process::exit(status);
      },
    }
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
  /// Failed to parse a value
  /// Returns error message from parsing
  ParseFromFailure(String),

  /// Missing value for a flag that expected one
  /// Specifies flag that was missing a value
  MissingValue(String),

  /// Help flag was passed, parsing stopped
  HelpRequested,

  /// Unknown flag was passed
  UnknownFlag(String),
}

use std::fmt;
impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{:?}", self) }
}
