use std::io;
use std::collections::HashMap;

fn main() {
    println!("Welcome to the Employee DB");
    let mut employee_department_map: HashMap<String, String> = HashMap::new();
    employee_department_map.insert("brent".to_string(), "engineering".to_string());
    employee_department_map.insert("kim".to_string(), "engineering".to_string());
    employee_department_map.insert("david".to_string(), "engineering".to_string());
    employee_department_map.insert("zoltan".to_string(), "engineering".to_string());
    employee_department_map.insert("aaron".to_string(), "engineering".to_string());
    employee_department_map.insert("bob".to_string(), "marketing".to_string());
    employee_department_map.insert("cat".to_string(), "marketing".to_string());
    employee_department_map.insert("zed".to_string(), "marketing".to_string());
    employee_department_map.insert("yel".to_string(), "marketing".to_string());
    employee_department_map.insert("ralf".to_string(), "marketing".to_string());
    employee_department_map.insert("sam".to_string(), "marketing".to_string());
    employee_department_map.insert("mitch".to_string(), "marketing".to_string());

    loop {
        println!("Please enter command:");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)
            .expect("Failed to read line");
        let args: Vec<&str> = user_input.trim().split(" ").collect();
        if args[0].to_string().to_lowercase() == "add".to_string() {
            if args.len() > 3 {
                employee_department_map = add_employee(args[1], args[3], employee_department_map);
            } else {
                println!("Please use: add {{Employee Name}} to {{Department Name}}")
            }
        } else if args[0].to_string().to_lowercase() == "list".to_string() {
            if args.len() > 1 {
                list_employees(Some(args[1]), &employee_department_map);
            } else {
                list_employees(None, &employee_department_map);
            }
        } else if args[0].to_string().to_lowercase() == "quit".to_string() {
            println!("Nerd!");
            break;
        } else {
            println!("Command not recognized, use add, list, or quit");
        };
        // println!("Employee listing: \n{:?}", employee_department_map);
    }
}

fn add_employee(name: &str, department: &str, mut map: HashMap<String, String>) -> HashMap<String, String> {
    map.insert(String::from(name.to_lowercase()), String::from(department.to_lowercase()));
    map
}

fn list_employees(department: Option<&str>, map: &HashMap<String, String>) {
    match department {
        Some(department) => {
            let mut employee_list: Vec<String> = vec![];
            for (employee, employee_department) in map {
                if employee_department.to_lowercase() == department.to_lowercase() {
                    employee_list.push(employee.to_string());
                }
            };
            println!("Employees in {} department:", word_to_uppercase(department));
            employee_list.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
            for employee in employee_list {
                println!("{}", word_to_uppercase(&employee))
            };
        },
        None => {
            let mut department_employee_map: HashMap<String, Vec<String>> = HashMap::new();
            println!("Full employee listing by department:");
            for (employee, department) in map {
                let employee_list = department_employee_map.entry(department.to_string()).or_insert(Vec::new());
                employee_list.push(employee.to_string());
            };
            for (department, mut employee_list) in department_employee_map {
                println!("Employees in {} department:", word_to_uppercase(&department));
                employee_list.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
                for employee in employee_list {
                    println!("{}", word_to_uppercase(&employee))
                };
            }
        },

    };
}

fn word_to_uppercase(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(first_char) => first_char.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
