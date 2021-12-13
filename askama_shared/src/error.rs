use std::fmt::{self, Display};

pub type Result<I> = ::std::result::Result<I, Error>;

/// askama error type
///
/// # Feature Interaction
///
/// If the feature `serde_json` is enabled an
/// additional error variant `Json` is added.
///
/// # Why not `failure`/`error-chain`?
///
/// Error from `error-chain` are not `Sync` which
/// can lead to problems e.g. when this is used
/// by a crate which use `failure`. Implementing
/// `Fail` on the other hand prevents the implementation
/// of `std::error::Error` until specialization lands
/// on stable. While errors impl. `Fail` can be
/// converted to a type impl. `std::error::Error`
/// using a adapter the benefits `failure` would
/// bring to this crate are small, which is why
/// `std::error::Error` was used.
///
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// formatting error
    Fmt(fmt::Error),
    RegEx(regex::Error),
    Chrono(chrono::format::ParseError),

    /// json conversion error
    #[cfg(feature = "serde_json")]
    Json(::serde_json::Error),

    /// yaml conversion error
    #[cfg(feature = "serde_yaml")]
    Yaml(::serde_yaml::Error),
}

impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            Error::Fmt(ref err) => err.source(),
            Error::RegEx(ref err) => err.source(),
            Error::Chrono(ref err) => err.source(),
            #[cfg(feature = "serde_json")]
            Error::Json(ref err) => err.source(),
            #[cfg(feature = "serde_yaml")]
            Error::Yaml(ref err) => err.source(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Fmt(ref err) => write!(formatter, "formatting error: {}", err),
            Error::RegEx(ref err) => write!(formatter, "regex error: {}", err),
            Error::Chrono(ref err) => write!(formatter, "chrono parse error: {}", err),
            #[cfg(feature = "serde_json")]
            Error::Json(ref err) => write!(formatter, "json conversion error: {}", err),
            #[cfg(feature = "serde_yaml")]
            Error::Yaml(ref err) => write!(formatter, "yaml conversion error: {}", err),
        }
    }
}

impl From<fmt::Error> for Error {
    fn from(err: fmt::Error) -> Self {
        Error::Fmt(err)
    }
}

impl From<regex::Error> for Error {
    fn from(err: regex::Error) -> Self {
        Error::RegEx(err)
    }
}

impl From<chrono::format::ParseError> for Error {
    fn from(err: chrono::format::ParseError) -> Self {
        Error::Chrono(err)
    }
}

#[cfg(feature = "serde_json")]
impl From<::serde_json::Error> for Error {
    fn from(err: ::serde_json::Error) -> Self {
        Error::Json(err)
    }
}

#[cfg(feature = "serde_yaml")]
impl From<::serde_yaml::Error> for Error {
    fn from(err: ::serde_yaml::Error) -> Self {
        Error::Yaml(err)
    }
}

#[cfg(test)]
mod tests {
    use super::Error;

    trait AssertSendSyncStatic: Send + Sync + 'static {}
    impl AssertSendSyncStatic for Error {}
}
