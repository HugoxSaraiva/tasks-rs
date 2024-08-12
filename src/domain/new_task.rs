use super::Scope;

#[derive(Debug)]
pub struct NewTask {
    pub description: String,
    pub scope: Option<Scope>,
}
