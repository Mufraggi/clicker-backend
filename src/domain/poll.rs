use derive_more::Display;
use crate::domain::id::Id;

#[derive(Display, Debug)]
pub struct Poll {}

pub type PollId = Id<Poll>;