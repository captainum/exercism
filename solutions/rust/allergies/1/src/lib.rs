pub struct Allergies {
    data: Vec<Allergen>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(mut score: u32) -> Self {
        let mut data = Vec::<Allergen>::new();

        let mut idx = 0;

        while score > 0 && idx < 8 {
            if score % 2 == 1 {
                data.push(
                    match idx {
                        0 => Allergen::Eggs,
                        1 => Allergen::Peanuts,
                        2 => Allergen::Shellfish,
                        3 => Allergen::Strawberries,
                        4 => Allergen::Tomatoes,
                        5 => Allergen::Chocolate,
                        6 => Allergen::Pollen,
                        _ => Allergen::Cats,
                    }
                );
            }
            score /= 2;
            idx += 1;
        }

        Allergies { data }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.data.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.data.clone()
    }
}