use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GroupState {
    pub all_on: bool,
    pub any_on: bool,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GroupAction {
    pub on: bool,
    pub toggle: Option<bool>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub state: GroupState,
    pub action: GroupAction,
}