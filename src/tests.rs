use crate::flag::{FlagSet, ParseError, Preset};

#[test]
fn test_basic() {
  let mut fs = FlagSet::new();
  let mut bool_option: Option<bool> = None;
  fs.add("opt", "test bool option", &mut bool_option);
  let mut int_option: Option<i32> = None;
  fs.add("num", "test num option", &mut int_option);
  let mut int_preset = Preset(5);
  fs.add("num_pre", "test_num_preset", &mut int_preset);
  assert!(fs
    .parse(
      vec!["-opt", "true", "--num", "3", "-num_pre", "4"]
        .into_iter()
        .map(String::from)
    )
    .is_ok());
  assert_eq!(bool_option, Some(true));
  assert_eq!(int_option, Some(3));
  assert_eq!(int_preset.into_inner(), 4);
}

#[test]
fn needs_help() {
  let mut fs = FlagSet::new();
  assert_eq!(
    fs.parse(vec!["-h"].into_iter().map(String::from)),
    Err(ParseError::HelpRequested)
  );
  assert_eq!(
    fs.parse(vec!["-help"].into_iter().map(String::from)),
    Err(ParseError::HelpRequested)
  );
}

#[test]
fn cannot_parse() {
  let mut fs = FlagSet::new();
  let mut bool_option: Option<bool> = None;
  fs.add("bool", "test bool option", &mut bool_option);
  assert!(fs
    .parse(vec!["--bool", "34"].into_iter().map(String::from))
    .is_err());
}
