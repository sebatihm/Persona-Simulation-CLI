pub struct Skill{
    name: String,
    atrribute: damage_type,
    damage: u32,
    sp_consumed: u32
}

impl Skill{

    pub fn new() -> Self{
        Self{
            name: String::from("Agi"),
            atrribute: damage_type::Fire,
            damage: 30,
            sp_consumed: 3
        }
    }

    pub fn get_atrribute(&self) -> &damage_type{
        &self.atrribute
    }

    pub fn get_name(&self) -> String{
        String::from(&self.name)
    }

    pub fn get_damage(&self) -> u32{
        self.damage
    }

    pub fn get_sp_consumed(&self) -> u32{
        self.sp_consumed
    }

}

#[derive(Debug,PartialEq, Eq)]
pub enum damage_type  {
    Slash,
    Strike,
    Pierce,
    Fire,
    Ice,
    Electric,
    Wind,
    Light,
    Dark,
    Ulmighty
}