macro_rules! scu_enum_err {
  {$($variant:ident($err:path),)*} => {
    #[derive(Debug)]
    pub enum ScuError {
      $($variant($err),)*
    }

    $(impl From<$err> for ScuError {
      fn from(value: $err) -> Self {
        ScuError::$variant(value)
      }
    })*

    impl std::fmt::Display for ScuError {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
          $(
            Self::$variant(err) => err.fmt(f),  
          )*
        }
      }
    }
  };
}

scu_enum_err! {
  IoError(std::io::Error),
  DeserializeError(toml::de::Error),
  SerializeError(toml::ser::Error),
}

pub type Result<T> = std::result::Result<T, ScuError>;