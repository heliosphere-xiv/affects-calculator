use ironworks::sestring::SeString;

use crate::schema::MetadataExtractor;

#[derive(Debug)]
pub struct NpcEquip {
    // pub model_main_hand: u64,
    // pub model_off_hand: u64,
    pub model_head: u32,
    pub model_body: u32,
    pub model_hands: u32,
    pub model_legs: u32,
    pub model_feet: u32,
    pub model_ears: u32,
    pub model_neck: u32,
    pub model_wrists: u32,
    pub model_left_ring: u32,
    pub model_right_ring: u32,
}

impl MetadataExtractor for NpcEquip {
    type Error = super::Error;

    fn name() -> String {
        "NpcEquip".into()
    }

    fn populate_row(row: ironworks::excel::Row) -> Result<Self, Self::Error> {
        let item = crate::populate!(
            row,
            // [model_main_hand, 0, into_u64],
            // [model_off_hand, 3, into_u64],
            [model_head, 6, into_u32],
            [model_body, 10, into_u32],
            [model_hands, 13, into_u32],
            [model_legs, 16, into_u32],
            [model_feet, 19, into_u32],
            [model_ears, 22, into_u32],
            [model_neck, 25, into_u32],
            [model_wrists, 28, into_u32],
            [model_left_ring, 31, into_u32],
            [model_right_ring, 34, into_u32],
        );

        Ok(item)
    }
}

impl NpcEquip {
    pub fn gear_models(&self) -> Vec<(u16, u8)> {
        let models = [
            // self.model_main_hand,
            // self.model_off_hand,
            self.model_head as u64,
            self.model_body as u64,
            self.model_hands as u64,
            self.model_legs as u64,
            self.model_feet as u64,
            self.model_ears as u64,
            self.model_neck as u64,
            self.model_wrists as u64,
            self.model_left_ring as u64,
            self.model_right_ring as u64,
        ];

        models
            .into_iter()
            .map(|combined| {
                let model_id = (combined & 0xFFFF) as u16;
                let variant_id = ((combined >> 16) & 0xFF) as u8;

                (model_id, variant_id)
            })
            .collect()
    }
}
