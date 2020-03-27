/// Flag related module.
/// Defines how to make something into a flag,
/// and how to parse arguments into a flag.
pub mod flag;
pub use flag::{FlagSet, Preset};
#[cfg(test)]
mod tests;
