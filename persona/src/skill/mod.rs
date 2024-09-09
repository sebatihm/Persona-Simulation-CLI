pub struct Skill{
    name: String,
    atrribute: AbilityType,
    points: u32,
    cost: u32
}

impl Skill{

    pub fn new() -> Self{
        Self{
            name: String::from("Agi"),
            atrribute: AbilityType::Fire,
            points: 30,
            cost: 3
        }
    }

    pub fn get_atrribute(&self) -> &AbilityType{
        &self.atrribute
    }

    pub fn get_name(&self) -> String{
        String::from(&self.name)
    }

    pub fn get_damage(&self) -> u32{
        self.points
    }

    pub fn get_sp_consumed(&self) -> u32{
        self.cost
    }

    pub fn agi() -> Self{
        Self { name: String::from("Agi"), atrribute: AbilityType::Fire, points: 50, cost: 3 }
    }

    pub fn agilao() -> Self{
        Self { name: String::from("Agilao"), atrribute: AbilityType::Fire, points: 100 , cost: 6 }
    }

    pub fn bufu() -> Self{
        Self { name: String::from("Bufu"), atrribute: AbilityType::Ice, points: 50, cost: 4 }
    }

    pub fn bufula() -> Self{
        Self { name: String::from("Bufula"), atrribute: AbilityType::Ice, points: 100 , cost: 8 }
    }
    pub fn zio() -> Self{
        Self { name: String::from("Zio"), atrribute: AbilityType::Electric, points: 50, cost: 4 }
    }

    pub fn zionga() -> Self{
        Self { name: String::from("Zionga"), atrribute: AbilityType::Electric, points: 100 , cost: 8 }
    }

    pub fn garu() -> Self{
        Self { name: String::from("Agi"), atrribute: AbilityType::Wind, points: 50, cost: 3 }
    }

    pub fn garula() -> Self{
        Self { name: String::from("Agilao"), atrribute: AbilityType::Wind, points: 100 , cost: 6 }
    }

    pub fn hama() -> Self{
        Self { name: String::from("Agi"), atrribute: AbilityType::Light, points: 50, cost: 6 }
    }

    pub fn mudo() -> Self{
        Self { name: String::from("Mudo"), atrribute: AbilityType::Dark, points: 100 , cost: 6 }
    }

    pub fn dia() -> Self{
        Self { name: String::from("Dia"), atrribute: AbilityType::Support, points: 50, cost: 6 }
    }

    pub fn diarama() -> Self{
        Self { name: String::from("Diarama"), atrribute: AbilityType::Support, points: 100 , cost: 6 }
    }

}

#[derive(Debug,PartialEq, Eq)]
pub enum AbilityType  {
    Slash,
    Strike,
    Pierce,
    Fire,
    Ice,
    Electric,
    Wind,
    Light,
    Dark,
    Ulmighty,
    Support
}

