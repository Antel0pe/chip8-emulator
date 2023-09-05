use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Configuration{
    #[serde(default = "default_ignore_y_in_8xy_shift_instruction")]
    pub ignore_y_in_8xy_shift_instruction: bool,
}

fn default_ignore_y_in_8xy_shift_instruction() -> bool{
    true
}