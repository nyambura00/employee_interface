//Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company.
//For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
use std::collections::HashMap;
use std::{io, string};

struct Employee{
    name: String,
    department: String,
}
fn main() {
    let mut employees = Vec::new();
    let mut full_house:HashMap<&str, &str> = HashMap::new();
    loop {
        println!("Enter a command: add_member, list_members, delete_member");
        let mut command: String = String::new();
        io::stdin().read_line(&mut command).expect("Failed to read line");
        let command = Some(command.trim());
        //use match pattern to determine the command
        match command {
            Some("add_member") => {
                println!("Enter the new name: ");
                let mut name: String = String::new();
                io::stdin().read_line(&mut name).expect( "Failed to read line");
                let name: &str = name.trim();
                println!("Enter a department: ");
                let mut department: String = String::new();
                io::stdin().read_line(&mut department).expect("Failed to read line");
                let department: &str = department.trim();
                //create a new employee and add it to the vector
                let new_employee = Employee{
                    name: name.to_string(),
                    department: department.to_string(),
                };
                employees.push(new_employee);
                //add the newest employee name, relative department to hashmap
                full_house.insert(name, department);
            },
            Some("list_members") => {
                println!("Enter a specific department: ");
                let mut department = String::new();
                io::stdin().read_line(&mut department).expect("Failed to read line");
                let department = department.trim();
                //use the hash map to get the employees in the department
                let employees = full_house.get(department).unwrap();
                println!("Employees in the department: {:?}", employees);
            },
            Some("delete_member") => {
                println!("Who is leaving?");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read line");
                let name = name.trim();
                //use hash map to find the employee and delete them from the hash map
                full_house.remove(name);
            },
            None => {
                println!("Goodbye!");
                break
            },  
        }
    }
}



        