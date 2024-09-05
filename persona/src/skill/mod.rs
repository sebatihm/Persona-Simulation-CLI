pub struct Skill{
    name: String,
    atrribute: String,
    damage: u32,
    sp_consumed: u32
}

impl Skill{

    pub fn new() -> Self{
        Self{
            name: String::from("Agi"),
            atrribute: String::from("Fire"),
            damage: 30,
            sp_consumed: 3
        }
    }

    pub fn get_atrribute(&self) -> String{
        String::from(&self.atrribute)
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