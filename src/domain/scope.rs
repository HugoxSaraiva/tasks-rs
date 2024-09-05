use std::fmt::Display;

#[derive(Debug, sqlx::Type)]
#[sqlx(transparent)]
pub struct Scope(String);

impl Scope {
    pub fn new(value: String) -> Self {
        Self(value.to_lowercase())
    }
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
