use crate::persona::*;
use crate::shadows::*;
pub struct Character{
    name: String,
    hp: u32,
    sp: u32,
    persona: Persona,
    max_hp:u32
}

impl Character{
    pub fn new() -> Self{
        Self{
            name: String::from("Makoto Yuki"),
            hp: 230,
            sp: 167,
            persona: Persona::new(),  
            max_hp: 230 
        }    
    }

    pub fn from(name: String, hp: u32,sp: u32, persona: Persona) -> Self{
        Self{
            name: name,
            hp: hp,
            sp: sp,
            persona: persona,
            max_hp: hp   
        }    
    }

    pub fn set_hp(&mut self, hp:u32){
        self.hp = hp;
    }

    pub fn get_hp(&mut self) -> u32{
        self.hp 
    }

    pub fn set_sp(&mut self, sp:u32){
        self.sp = sp;
    }

    pub fn get_sp(&mut self) -> u32{
        self.sp 
    }

    pub fn add_persona(&mut self,persona: Persona){
        self.persona = persona;
    }

    pub fn attack(&self, enemy: &mut Shadow){
        let health = enemy.get_hp();
        if health < 30{
            enemy.set_hp(0);
        } else {
            enemy.set_hp(health-30);
        }
        
        println!("The enemy is at {} hp", enemy.get_hp());
    }

    pub fn use_skill(&mut self,enemy: &mut Shadow){

        if enemy.get_weakness() == self.persona.get_skill().get_atrribute(){
            println!("Critical Damage");
            let health = enemy.get_hp();

            if health < (self.persona.get_skill().get_damage()*2){
                enemy.set_hp(0);
            } else {
                enemy.set_hp(health-((self.persona.get_skill().get_damage())*2));
            }            

        } else {
            let health = enemy.get_hp();

            if health < self.persona.get_skill().get_damage(){
                enemy.set_hp(0);
            } else {
                enemy.set_hp(health-self.persona.get_skill().get_damage());
            }
        }

        if self.sp < self.persona.get_skill().get_sp_consumed() {
            self.set_sp(0);
        } else {
            self.set_sp(self.sp - (self.persona.get_skill().get_sp_consumed()));
        }
                
        println!("The enemy is at {} hp", enemy.get_hp());
        println!(":::: {} sp remaining ::..", self.sp);
    }

    pub fn recover(&mut self){
        if self.hp + 35 >= self.max_hp{
            println!("{} uses Dia and recovers totally", self.persona.get_name());
            self.hp = self.max_hp;
        }else{
            println!("{} uses Dia and recovers 35", self.persona.get_name());
            self.hp = self.hp + 35;
        }
        println!(":::: Current hp: {}", self.hp);
        
    }

    pub fn check(&self, enemy: &Shadow){
        enemy.info();
    }

    pub fn get_persona(&self) -> &Persona{
        &self.persona
    }

    pub fn get_name(&self) -> String{
        String::from(&self.name)
    }


    
}