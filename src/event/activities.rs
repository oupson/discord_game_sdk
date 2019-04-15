use crate::{Action, Activity, User};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Join {
    pub secret: String,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Spectate {
    pub secret: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Request {
    pub user: User,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Invite {
    pub action: Action,
    pub user: User,
    pub activity: Activity,
}
