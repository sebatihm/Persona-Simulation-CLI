use crate::persona::*;
pub struct Character{
    name: String,
    hp: u32,
    sp: u32,
    persona: Persona
}

impl Character{
    pub fn character_default() -> Self{
        Self{
            name: String::from("Makoto Yuki"),
            hp: 230,
            sp: 167,
            persona: Persona::persona_default()   
        }

        
    }

    pub fn set_hp(&mut self, hp:u32){
        self.hp = self.hp + hp;
    }

    pub fn get_hp(&mut self) -> u32{
        self.hp 
    }

    pub fn set_sp(&mut self, sp:u32){
        self.sp = self.sp + sp;
    }

    pub fn get_sp(&mut self, hp:u32) -> u32{
        self.sp 
    }

    pub fn add_persona(&mut self,persona: Persona){
        self.persona = persona;
    }

    pub fn attack(&mut self, character: &mut Character){
        let health = character.get_hp();
        character.set_hp(health-30);
    }

    pub fn recover(&mut self){
        println!("{} uses Dia and recovers 35", self.persona.get_name());
        self.hp = self.hp + 50;
    }

    pub fn check(persona: &Persona){
        println!("The weakness is: {} !",persona.get_weakness());
    }

    
}