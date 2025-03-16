use binrw::{BinRead, binrw};
use ironworks::file::File;

#[derive(Debug)]
#[binrw]
#[br(little)]
pub struct RawImcFile {
    pub count: u16,
    pub part_mask: u16,
    #[br(count = part_mask.count_ones())]
    pub default_variants: Vec<ImageChangeData>,
    #[br(count = part_mask.count_ones() * count as u32)]
    pub variants: Vec<ImageChangeData>,
}

#[derive(Debug)]
#[binrw]
#[br(little)]
pub struct ImageChangeData {
    pub material_id: u8,
    pub decal_id: u8,
    pub attribute_and_sound: u16,
    pub vfx_id: u8,
    pub material_animation_id_mask: u8,
}

// impl ImageChangeData {
//     pub fn attribute_mask(&self) -> u16 {
//         self.attribute_and_sound & 0x3ff
//     }

//     pub fn sound_id(&self) -> u16 {
//         self.attribute_and_sound & 0xfc00
//     }

//     pub fn material_animation_id(&self) -> u8 {
//         self.material_animation_id_mask & 0xf
//     }
// }

#[derive(Debug)]
pub struct ImcFile {
    // pub count: u16,
    // pub part_mask: u16,
    pub parts: Vec<ImageChangeParts>,
}

impl ImcFile {
    pub fn try_from_raw(mut value: RawImcFile) -> Option<Self> {
        let mut parts = Vec::with_capacity(value.default_variants.len());

        for default in value.default_variants {
            let part = ImageChangeParts {
                default_variant: default,
                variants: Default::default(),
            };

            parts.push(part);
        }

        value.variants.reverse();

        for _ in 0..value.count {
            for part in &mut parts {
                let variant = value.variants.pop()?;
                part.variants.push(variant);
            }
        }

        Some(Self {
            // count: value.count,
            // part_mask: value.part_mask,
            parts,
        })
    }
}

#[derive(Debug)]
pub struct ImageChangeParts {
    pub default_variant: ImageChangeData,
    pub variants: Vec<ImageChangeData>,
}

impl File for RawImcFile {
    fn read(mut stream: impl ironworks::FileStream) -> std::result::Result<Self, ironworks::Error> {
        <RawImcFile as BinRead>::read(&mut stream)
            .map_err(|e| ironworks::Error::Resource(Box::new(e)))
    }
}
