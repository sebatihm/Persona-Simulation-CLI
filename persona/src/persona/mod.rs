use crate::skill::*;
pub struct Persona{
    name: String,
    arcana: String,
    weakness: String,
    skill: skill,
}

impl Persona{
    pub fn persona(name: String,arcana: String,weakness: String,skill: skill) -> Self{
        
        Self {
            name:name,
            arcana: arcana,
            skill: skill,
            weakness: weakness
        }

    }

    pub fn persona_default() -> Self{
        Self {
            name:String::from("Oprpheus"),
            arcana: String::from("Fool"),
            skill: skill::skill_default(),
            weakness: String::from("Wind")
        }

    }

    pub fn info(&self){
        println!("Persona: {}\nArcana: {}\nWeakness: {}",self.name,self.arcana,self.weakness);
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_weakness(&self) -> &str{
        &self.weakness
    }

    // pub fn set_name(&mut self, named: String){
    //     self.name = named;
    // }
    // Idk if should implement
    

}