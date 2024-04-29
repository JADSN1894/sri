pub(super) struct AirtifactPathSri(String);

impl From<AirtifactPathSri> for String {
  fn from(value: AirtifactPathSri) -> Self {
    value.0
  }
}

impl From<&String> for AirtifactPathSri {
  fn from(value: &String) -> Self {
    Self(value.to_owned())
  }
}
