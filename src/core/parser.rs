pub fn parse_string<'a,T: serde::Deserialize<'a>>(s: String) -> Result<T, serde_json::Error> {
  serde_json::from_str::<T>(s.as_str())
}
