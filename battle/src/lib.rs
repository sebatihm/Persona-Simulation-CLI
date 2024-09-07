pub mod battles{
    use persona::character::*;
    use persona::shadows::*;
    use std::io;
    use rand::Rng;

    pub fn battle(hero: &mut Character,shadow: &mut Shadow){
        let mut hero = hero;
        let mut shadow = shadow;
        println!("------------------------------------------------------------------------------------");
        println!("::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::");
        println!("------------------------------------------------------------------------------------\n\n");

        println!("Battle!");

        loop {
            let option = turn();

            println!("-----------------------------------------------------");
            if option ==  'A'{
                hero.use_skill(&mut shadow);
            }else if option == 'B'{
                hero.attack(&mut shadow);
            }else if option == 'C' {
                hero.get_persona().info();
                continue;
            }else if option == 'D' {
                hero.recover();
            }else if option == 'E'{
                println!("{} waits ...", hero.get_name());
            }else if option == 'F'{
                hero.check(&shadow);
                continue;
            }else if option == 'G'{
                println!("You cant escape");
                println!("Fight!");
            }else {
                println!("You didn't choose a valid option you lose a turn");  
            }
            println!("-----------------------------------------------------\n");

            if shadow.get_hp() == 0{
                println!("\n\n\n ....:::: Congratulations ::::.... ");
                println!(" You have won the battle\n\n\n\n");
                break;
            }

            println!("---------------- OPOUNENT TURN -----------------------");

            let option_choice: u32 =  rand::thread_rng().gen_range(1..=4);

            if option_choice == 1{
                shadow.recover();
            }else if option_choice == 2{
                shadow.use_skill(&mut hero);
            }else {
                shadow.attack(&mut hero);
            }
            println!("-----------------------------------------------------\n");


            if hero.get_hp() == 0{
                println!(" ....:::: Defeat ::::.... ");
                println!(" You've been defeated");
                break;
            }
            
            
        }

    }

    pub fn turn() -> char{
        let mut input = String::new();
        println!(":::: Command ::................................... ");
        println!("(A) Skill");
        println!("(B) Attack");
        println!("(C) Persona");
        println!("(D) Recover");
        println!("(E) Guard");
        println!("(F) Check");
        println!("(G) Flee");
        println!("---------------------");
        println!("Insert command: ");
            
        io::stdin()
            .read_line(&mut input)
            .expect("Something went wrong")
        ;
    
        input = input.to_uppercase();
    
        let option: char = match input.chars().next() {
            None => {
                let var_name = 'E';
                var_name
            },
            Some(c) =>  c
            
        };

        println!("---------------------");
        println!(":::::::::::::::................................... \n");


    
        option
        // let option = String::from(&input[..1]);
    
    }
    



}
