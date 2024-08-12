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
