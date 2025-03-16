use crate::schema::MetadataExtractor;

#[derive(Debug)]
pub struct ENpcBase {
    pub row_id: u32,
    pub model_chara: u16,
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

impl MetadataExtractor for ENpcBase {
    type Error = super::Error;

    fn name() -> String {
        "ENpcBase".into()
    }

    fn populate_row(row: ironworks::excel::Row) -> Result<Self, Self::Error> {
        let item = crate::populate!(
            row,
            [model_chara, 35, into_u16],
            // [model_main_hand, 65, into_u64],
            // [model_off_hand, 68, into_u64],
            [model_head, 71, into_u32],
            [model_body, 75, into_u32],
            [model_hands, 78, into_u32],
            [model_legs, 81, into_u32],
            [model_feet, 84, into_u32],
            [model_ears, 87, into_u32],
            [model_neck, 90, into_u32],
            [model_wrists, 93, into_u32],
            [model_left_ring, 96, into_u32],
            [model_right_ring, 99, into_u32],
            row_id: row.row_id(),
        );

        Ok(item)
    }
}

impl ENpcBase {
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
