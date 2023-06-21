use std::{path, borrow::Cow};

pub fn stringify(path: impl AsRef<path::Path>, separator: impl AsRef<str>) -> String {
  let components = path.as_ref().components().map(|c| match c {
    path::Component::Prefix(prefix) => (
      Cow::from(prefix.as_os_str().to_string_lossy().to_string().replace("\\\\?\\", "")), false
    ),
    path::Component::RootDir => (Cow::from(""), true),
    path::Component::CurDir => (Cow::from("."), true),
    path::Component::ParentDir => (Cow::from(".."), true),
    path::Component::Normal(part) => (part.to_string_lossy(), true),
  });

  components.reduce(
    |a, b| (format!("{}{}{}", a.0, if a.1 { separator.as_ref() } else { "" }, b.0).into(), b.1)
  ).map(|x| x.0.to_string()).unwrap_or_default()
}

pub fn stringify_default(path: impl AsRef<path::Path>) -> String {
  stringify(path, path::MAIN_SEPARATOR.to_string())
}