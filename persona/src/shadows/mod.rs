use crate::skill::*;
use crate::character::*;
pub struct Shadow{
    name: String,
    arcana: String,
    weakness: damage_type,
    skill: Skill,
    hp: u32,
    max_hp: u32
}

impl Shadow{
    pub fn from(name: String,arcana: String,weakness: damage_type,skill: Skill, hp:u32) -> Self{
        
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
            weakness: damage_type::Fire,
            hp: 100,
            max_hp: 100
        }

    }

    pub fn info(&self){
        println!("..:: This is the information you seem to see ::.. \nName: {}\nArcana: {}\nWeakness: {:?}",self.name,self.arcana,self.weakness);
        println!("..:::::::::::::::::::::::::::::::::::::::::::..\n");
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

    pub fn get_weakness(&self) -> &damage_type{
        &self.weakness
    }

    pub fn get_skill(&self) -> &Skill{
        &self.skill
    }

    pub fn recover(&mut self){
        if self.hp + 35 >= self.max_hp{
            println!("{} uses Dia and recovers totally", self.name);
            self.hp = self.max_hp;
        }else{
            println!("{} uses Dia and recovers 35", self.name);
            self.hp = self.hp + 35;
        }
        
    }

    pub fn use_skill(&self,enemy: &mut Character){

        if enemy.get_persona().get_weakness() == self.get_skill().get_atrribute(){
            println!("Critical");
            let health = enemy.get_hp();

            if health < (self.get_skill().get_damage()*2){
                enemy.set_hp(0);
            } else {
                enemy.set_hp(health-(self.get_skill().get_damage()* 2));
            }

        } else {
            let health = enemy.get_hp();

            if health < self.get_skill().get_damage(){
                enemy.set_hp(0);
            } else {
                enemy.set_hp(health-self.get_skill().get_damage());
            }
        }
                
        println!("You get damage :::::.. \n:::: Current hp {}", enemy.get_hp());
    }

    pub fn attack(&self, enemy: &mut Character){
        let health = enemy.get_hp();
        if health < 30{
            enemy.set_hp(0);
        } else {
            enemy.set_hp(health-30);
        }
        
        println!("You get damage :::::.. \n:::: Current hp: {}", enemy.get_hp());
    }

    

}