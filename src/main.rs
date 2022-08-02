//Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company.
//For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
use std::collections::HashMap;
use std::io;

/*struct Employee{
    name: String,
    department: String,
}*/
fn main() {
    //hashmap of employee name, department
    let mut employees = HashMap::new();
    loop {
        println!("Enter a command: ");
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to read line");
        let command = command.trim().to_string();
        if command == "quit" {
            break;
        }
        let mut split_command = command.split_whitespace();
        let command = split_command.next().unwrap();

        //pattern match add, list and delete employees
        match command {
            "add" => {
                let name = split_command.next().unwrap();
                let department = split_command.next().unwrap();
                employees.insert(name.to_string(), department.to_string());
                //print new employee
                println!("{} added to {}", name, department);
            }
            "list" => {
                let specific_department = split_command.next().unwrap();
                let mut employees_in_department = Vec::new();
                for (name, department) in employees.iter() {
                    if department == specific_department {
                        employees_in_department.push(name.to_string());
                    }
                }
                employees_in_department.sort();
                for name in employees_in_department {
                    println!("{}", name);
                }
            }
            "delete" => {
                let name = split_command.next().unwrap();
                employees.remove(name);
            }
            _ => {
                println!("Invalid command");
            }
        }
    }
}



        