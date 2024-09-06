use persona::character::*;
use persona::shadows::*;
use std::io;

fn main() {
    let mut hero = Character::new();
    let mut shadow = Shadow::new();

    println!("Memento Mori");
    println!("Battle!");
    println!(".-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.");

    loop {
        let option = turn();

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

        if hero.get_hp() == 0{
            println!(" ....:::: Defeat ::::.... ");
            println!(" You've been defeated");
            break;
        }else if shadow.get_hp() == 0{
            println!("\n\n\n ....:::: Congratulations ::::.... ");
            println!(" You have won the battle\n\n\n\n");
            break;
        }


    }
}

fn turn() -> char{
    let mut input = String::new();
    println!("..:: Command ::.. ");
    println!("(A) Skill");
    println!("(B) Attack");
    println!("(C) Persona");
    println!("(D) Recover");
    println!("(E) Guard");
    println!("(F) Check");
    println!("(G) Flee");
    println!("'':: ::::::: ::''");

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

    option
    // let option = String::from(&input[..1]);

}
