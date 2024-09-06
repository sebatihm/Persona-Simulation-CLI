use crate::skill::*;
pub struct Shadow{
    name: String,
    arcana: String,
    weakness: String,
    skill: Skill,
    hp: u32,
    max_hp: u32
}

impl Shadow{
    pub fn from(name: String,arcana: String,weakness: String,skill: Skill, hp:u32) -> Self{
        
        Self {
            name:name,
            arcana: arcana,
            skill: skill,
            weakness: weakness,
            hp: hp,
            max_hp: hp
        }

    }

    pub fn new() -> Self{
        Self {
            name:String::from("Cowardly Maya"),
            arcana: String::from("Fool"),
            skill: Skill::new(),
            weakness: String::from("Agi"),
            hp: 100,
            max_hp: 100
        }

    }

    pub fn info(&self){
        println!("..:: This is the information you seem to see ::.. \nName: {}\nArcana: {}\nWeakness: {}",self.name,self.arcana,self.weakness);
    }

    pub fn set_hp(&mut self, hp:u32){
        self.hp = hp;
    }

    pub fn get_hp(&mut self) -> u32{
        self.hp 
    }

    pub fn get_name(&self) -> String{
        String::from(&self.name)
    }

    pub fn get_weakness(&self) -> String{
        String::from(&self.weakness)
    }

    pub fn get_skill(&self) -> &Skill{
        &self.skill
    }

    

}