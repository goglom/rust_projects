use std::collections::HashMap;
use std::io;

fn try_to_add(
    args: Vec<&str>,
    departments: &mut HashMap<String, String>,
) -> Result<String, String> {
    if args.len() < 3 {
        return Err("Too few arguments for 'add' command".to_string());
    }

    let mut splitted = args.split(|&x| x == "to");

    let name = splitted.next().unwrap().join(" ");

    let department = splitted
        .next()
        .ok_or("Expected 'to' as separator")?
        .join(" ");

    match departments.insert(name.clone(), department.clone()) {
        None => Ok(format!(
            "> Added person '{}' to '{}' department",
            name, department
        )),
        Some(prev_dep) => Ok(format!(
            "> Person '{}' moved from '{}' to '{}' department",
            name, prev_dep, department
        )),
    }
}

fn print_list(args: Vec<&str>, departments: &HashMap<String, String>) {
    match args.first() {
        None => {
            let mut deps: Vec<(&String, &String)> = departments.iter().collect();
            deps.sort_by(|x1, x2| x1.0.cmp(x2.0));

            for (person, dep) in deps {
                println!("'{}' in '{}'", person, dep);
            }


        }
        Some(department) => {
            let x = departments.iter().filter(|(_, v)| v == department);
            
            if x.clone().next().is_none(){
                return println!("Department '{}' is not registred yet", department);
            }

            println!("> Persons of the '{}' department:", department);
            for (person, _) in  x {
                println!("  {}", person);
            }
        }
    }
}

fn main() {
    let mut departments: HashMap<String, String> = HashMap::new();

    loop {
        let mut line = String::new();
        if let Err(err) = io::stdin().read_line(&mut line) {
            print!("Something went wrong wile reading sdtdin: {err}");
            break;
        }
        let line = line.to_ascii_lowercase();
        let mut iter = line.trim().split(' ');

        if let Some(fst) = iter.next() {
            match fst {
                "exit" => return,
                "add" => {
                    println!(
                        "{}",
                        try_to_add(iter.collect(), &mut departments)
                            .unwrap_or_else(|err| format!("> Error: {err}"))
                    )
                }
                "list" => print_list(iter.collect(), &departments),
                "" => (),
                _ => println!("> Unknown command: '{fst}'"),
            }
        }
    }
}
