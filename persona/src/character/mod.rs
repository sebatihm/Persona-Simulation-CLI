use persona::persona::*;
pub struct character{
    name: String,
    hp: u32,
    sp: u32,
    personas: Vec<persona>
}

impl persona{
    pub fn character_default() -> Self{
        Self{
            name: String::from("Makoto Yuki"),
            hp: 230,
            sp: 167,
            personas: Vec::new(),   
        }

        
    }

    pub fn add_persona(){

    }

    
}