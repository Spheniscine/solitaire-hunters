use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, FromRepr};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Default)]
pub struct Skin;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, EnumIter, strum_macros::Display, Default, FromRepr)]
#[repr(u8)]
pub enum ColorMode {
    #[default] Dark, Light
}