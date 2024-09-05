pub struct skill{
    name: String,
    atrribute: String,
    damage: u32
}

impl skill{

    pub fn skill_default() -> Self{
        Self{
            name: String::from("Agi"),
            atrribute: String::from("Fire"),
            damage: 30
        }
    }

}