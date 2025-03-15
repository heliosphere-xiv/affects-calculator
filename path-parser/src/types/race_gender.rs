#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Race {
    AuRa,
    Elezen,
    Highlander,
    Hrothgar,
    Lalafell,
    Midlander,
    Miqote,
    Roegadyn,
    Viera,
}

impl std::fmt::Display for Race {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pretty = match self {
            Self::AuRa => "Au Ra",
            Self::Elezen => "Elezen",
            Self::Highlander => "Highlander",
            Self::Hrothgar => "Hrothgar",
            Self::Lalafell => "Lalafell",
            Self::Midlander => "Midlander",
            Self::Miqote => "Miqo'te",
            Self::Roegadyn => "Roegadyn",
            Self::Viera => "Viera",
        };

        write!(f, "{pretty}")
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Gender {
    Female,
    Male,
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pretty = match self {
            Self::Female => "Female",
            Self::Male => "Male",
        };

        write!(f, "{pretty}")
    }
}
