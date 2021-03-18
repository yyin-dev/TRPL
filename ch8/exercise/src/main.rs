use std::collections::HashMap;

fn main() {
    // let v = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 10, 10];
    // let (avg, median, mode) = get_stat(&v);
    // println!("Avg: {}, median: {}, mode: {}", avg, median, mode);

    department_database();
}

fn get_stat(v: &Vec<i32>) -> (i32, i32, i32) {
    let mut v_mut = v.clone();
    v_mut.sort();

    let mut avg = 0;
    let mut map = HashMap::new();
    for i in v_mut.iter() {
        let count = map.entry(*i).or_insert(0);
        *count += 1;

        avg += i;
    }

    let mut mode = 0;
    let mut curr_max_count = 0;
    for (k, v) in &map {
        if *v > curr_max_count {
            curr_max_count = *v;
            mode = *k;
        }
    }

    (avg / (v.len() as i32), v_mut[v.len() / 2], mode)
}

fn department_database() {
    use std::io;
    println!("Welcome to the department database!");
    println!("The database has 3 commands: ");
    println!("1: Add employee to deaprtment;");
    println!("2: List employees in one deaprtment;");
    println!("2: List all employees in the company;");

    let mut db = HashMap::new();

    loop {
        println!("Enter the command number (1, 2, 3): ");

        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Canont read input");

        match command.trim() {
            "1" => {
                let mut clause = String::new();
                io::stdin()
                    .read_line(&mut clause)
                    .expect("Canont read input");

                // Check if input is valid: Add Name to Department
                let words: Vec<&str> = clause.trim().split(" ").collect();
                if words.len() < 4 || words[0] != "Add" || words[2] != "to" {
                    println!("Invalid input, enter again!");
                    continue;
                }

                let employee = words[1];
                let department = words[3];
                let existing = db.entry(String::from(department)).or_insert(Vec::new());
                existing.push(String::from(employee));
            }
            "2" => {
                let mut clause = String::new();
                io::stdin()
                    .read_line(&mut clause)
                    .expect("Canont read input");

                let employees = match db.get(clause.trim()) {
                    Some(ref vec) => format!("{:?}", vec), // a reference
                    None => String::from("[]"),
                };
                println!("{}", employees)
            }
            "3" => {
                // sort keys
                let mut departments = Vec::new();
                for d in db.keys() {
                    departments.push(String::from(d));
                }
                departments.sort();

                // print in order
                for d in departments {
                    if let Some(ref vec) = db.get(&d) {
                        println!("{}: {:?}", d, vec);
                    }
                }
            }
            _ => {
                println!("Invalid command, enter again!");
                continue;
            }
        }
    }
}
