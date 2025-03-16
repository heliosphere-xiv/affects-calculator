pub mod file_or_part;
pub mod model_info;
pub mod race_gender;
pub mod skeleton_slot;

pub use self::{
    file_or_part::{FileOrPart, file_or_part},
    model_info::{ModelInfo, ModelKind, model_info, model_info_with_raw},
    race_gender::{Gender, Race},
    skeleton_slot::{SkeletonSlot, skeleton_slot},
};

enum_str! {
    pub enum Language {
        English => "en",
        Japanese => "ja",
        German => "de",
        French => "fr",
    }
}
