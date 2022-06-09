pub trait sounds {
    fn frequencies(&self) -> Vec<u16>;
}

pub struct Vowel {
    pub character:   &str;
    pub formant_1:   u16;
    pub formant_2:   u16;
    pub length:      u16;
    pub diacritics:  Vec<Diacritic>;
    pub frequencies: Vec<u16>;
}

impl sounds for Vowel {
    fn frequencies(&self) -> Vec<u16> {
        let self::frequencies = Vec::new();
        
        match self::character {
        
        "i" => self::formant_1 = 240 && self::formant_2 = 2400,
        "y" => self::formant_1 = 235 && self::formant_2 = 2100,
        "e" => self::formant_1 = 390 && self::formant_2 = 2300,
        "ø" => self::formant_1 = 370 && self::formant_2 = 1900,
        "ɛ" => self::formant_1 = 610 && self::formant_2 = 1900,
        "œ" => self::formant_1 = 585 && self::formant_2 = 1710,
        "a" => self::formant_1 = 850 && self::formant_2 = 1610,

        }

        self::frequencies.push(self::formant_1);
        self::frequencies.push(self::formant_2);

        self::frequencies
    }
}

pub struct Consonant {
    pub character:   &str;
    pub place:       &str;
    pub manner:      &str;
    pub length:      u16;
    pub diacritics:  Vec<Diacritic>;
    pub frequencies: Vec<u16>;
}

pub struct Diacritic {
    pub character:   &str;
    pub frequencies: Vec<u16>;
}
