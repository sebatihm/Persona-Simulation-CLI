use crate::skill::*;
pub struct Persona{
    name: String,
    arcana: String,
    weakness: AbilityType,
    skill: Skill,
}

impl Persona{
    pub fn from(name: String,arcana: String,weakness: AbilityType,skill: Skill) -> Self{
        
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
            skill: Skill::agi(),
            weakness: AbilityType::Wind
        }

    }

    pub fn info(&self){
        println!(":::::::::::::::::::::::::::");

        println!("Persona: {}\nArcana: {}\nWeakness: {:?}",self.name,self.arcana,self.weakness);

        println!(":::::::::::::::::::::::::::\n");
    }

    pub fn get_name(&self) -> String{
        String::from(&self.name)
    }

    pub fn get_weakness(&self) -> &AbilityType{
        &self.weakness
    }

    pub fn get_skill(&self) -> &Skill{
        &self.skill
    }

    

}