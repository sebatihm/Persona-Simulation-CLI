use crate::skill::*;
use std::io;
pub struct Persona{
    name: String,
    arcana: String,
    weakness: AbilityType,
    skills: Vec<Skill>,
}

impl Persona{
    pub fn from(name: String,arcana: String,weakness: AbilityType,skill: Vec<Skill>) -> Self{
        
        Self {
            name:name,
            arcana: arcana,
            skills: skill,
            weakness: weakness
        }

    }

    pub fn new() -> Self{
        Self {
            name:String::from("Orpheus"),
            arcana: String::from("Fool"),
            skills: vec![Skill::agi()],
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
        

        /* `&skill::Skill` value */
        
        &self.skills[1]
        
    }

    pub fn choose_skill(self){
        let mut i = 1;
        println!("This are the skills that {} have:",self.get_name());

        for element in &self.skills{
            let type_of_cost = if element.get_atrribute() != &AbilityType::Pierce || element.get_atrribute() != &AbilityType::Slash || element.get_atrribute() != &AbilityType::Strike{
                String::from("sp")
            } else { String::from("hp")};

            if element.get_atrribute() != &AbilityType::Support{
                let type_of_damage = if element.get_damage() <= 50 {String::from("weak")} else {String::from("medium")};
                println!(" ({}) {} consumes {} {} and {} damage", element.get_name(), element.get_sp_consumed(),i,type_of_cost,type_of_damage);

            } else {
                let type_of_damage = if element.get_damage() <= 50 {String::from("a little")} else {String::from("mostly health")};
                println!("({}) {} consumes {} {} and recovers {} health", element.get_name(), element.get_sp_consumed(),i,type_of_cost,type_of_damage);
            }
            i += 1;
        }

        println!("What ability do you want to use? \n(Press any other key to cancel)");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Something went wrong")
        ;
    }

    

}