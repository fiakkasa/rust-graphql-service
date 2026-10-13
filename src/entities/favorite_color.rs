use strum::{Display, EnumString, VariantNames};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Display, EnumString, VariantNames)]
pub enum FavoriteColor {
    Red,
    Blue,
    Green,
}

impl FavoriteColor {
    pub(crate) fn allowed_values() -> String {
        Self::VARIANTS.join(", ")
    }
}
