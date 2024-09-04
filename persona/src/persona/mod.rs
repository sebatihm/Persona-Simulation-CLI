pub struct Persona{
    name: String,
    arcana: String,
    weakness: String,
    skill: String,
    hp: u32,
    sp: u32
}

impl Persona{
    pub fn persona(name: String,arcana: String,weakness: String,skill: String, hp: u32,sp: u32) -> Self{
        
        Self {
            name:name,
            arcana: arcana,
            hp: hp,
            sp: sp,
            skill: skill,
            weakness: weakness
        }

    }
    pub fn attack(&self, persona: &mut Persona){
        println!("The persona {} uses {}.", self.name,self.skill);

        if persona.weakness == self.skill{
            print!("Critical Damage");
            persona.hp = persona.hp - 50;
        } else {
            persona.hp = persona.hp - 20;
        }

        
    }

    pub fn recover(&mut self){
        println!("{} uses Dia and recovers 35", self.name);
        self.hp = self.hp + 50;
    }
    pub fn info(&self){
        println!("Persona: {}\nArcana: {}\nWeakness: {}\nSkill: {}\nHp remaining: {}\nSp remaining: {}",self.name,self.arcana,self.weakness,self.skill,self.hp,self.sp);
    }

    pub fn check(persona: &Persona){
        println!("The weakness is: {} !",persona.weakness);
    }


}