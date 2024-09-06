use crate::skill::*;
pub struct Persona{
    name: String,
    arcana: String,
    weakness: String,
    skill: Skill,
}

impl Persona{
    pub fn from(name: String,arcana: String,weakness: String,skill: Skill) -> Self{
        
        Self {
            name:name,
            arcana: arcana,
            skill: skill,
            weakness: weakness
        }

    }

    pub fn new() -> Self{
        Self {
            name:String::from("Orpheus"),
            arcana: String::from("Fool"),
            skill: Skill::new(),
            weakness: String::from("Wind")
        }

    }

    pub fn info(&self){
        println!("Persona: {}\nArcana: {}\nWeakness: {}",self.name,self.arcana,self.weakness);
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

    // pub fn set_name(&mut self, named: String){
    //     self.name = named;
    // }
    // Idk if should implement
    

}