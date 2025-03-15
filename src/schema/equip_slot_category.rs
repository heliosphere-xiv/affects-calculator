use crate::schema::MetadataExtractor;

#[derive(Debug)]
pub struct EquipSlotCategory {
    pub main_hand: i8,
    pub off_hand: i8,
    pub head: i8,
    pub body: i8,
    pub gloves: i8,
    pub waist: i8,
    pub legs: i8,
    pub feet: i8,
    pub ears: i8,
    pub neck: i8,
    pub wrists: i8,
    pub finger_l: i8,
    pub finger_r: i8,
    pub soul_crystal: i8,
}

impl MetadataExtractor for EquipSlotCategory {
    type Error = super::Error;

    fn name() -> String {
        "EquipSlotCategory".into()
    }

    fn populate_row(row: ironworks::excel::Row) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let item = crate::populate!(
            row,
            [main_hand, 0, into_i8],
            [off_hand, 1, into_i8],
            [head, 2, into_i8],
            [body, 3, into_i8],
            [gloves, 4, into_i8],
            [waist, 5, into_i8],
            [legs, 6, into_i8],
            [feet, 7, into_i8],
            [ears, 8, into_i8],
            [neck, 9, into_i8],
            [wrists, 10, into_i8],
            [finger_l, 11, into_i8],
            [finger_r, 12, into_i8],
            [soul_crystal, 13, into_i8],
        );

        Ok(item)
    }
}

impl std::fmt::Display for EquipSlotCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut slots = Vec::with_capacity(1);
        let mut blocks = Vec::with_capacity(2);

        let mut fill = |pretty: &'static str, value: i8| {
            if value == 1 {
                slots.push(pretty);
            }

            if value == -1 {
                blocks.push(pretty);
            }
        };

        fill("main hand", self.main_hand);
        fill("off hand", self.off_hand);
        fill("head", self.head);
        fill("body", self.body);
        fill("gloves", self.gloves);
        fill("waist", self.waist);
        fill("legs", self.legs);
        fill("feet", self.feet);
        fill("ears", self.ears);
        fill("neck", self.neck);
        fill("wrists", self.wrists);
        fill("left finger", self.finger_l);
        fill("right finger", self.finger_r);
        fill("soul crystal", self.soul_crystal);

        let slots_empty = slots.is_empty();
        let blocks_empty = blocks.is_empty();
        match (slots_empty, blocks_empty) {
            (true, true) => write!(f, "none"),
            (true, false) => write!(f, "none (blocks {})", blocks.join(", ")),
            (false, true) => write!(f, "{}", slots.join(", ")),
            (false, false) => write!(f, "{} (blocks {})", slots.join(", "), blocks.join(", ")),
        }
    }
}
