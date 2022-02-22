use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::io::Write;

type EmployeeDatabse = HashMap<String, Vec<String>>;

enum Command {
    AddEmployeeToDepartment {
        employee: String,
        department: String,
    },
    ListEmployeesByDepartment {
        department: String,
    },
    ListEmployees,
}

#[must_use]
fn get_command() -> Command {
    'get_input: loop {
        print!("> ");
        io::stdout().flush().ok();

        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() {
            continue 'get_input;
        }
        let line = line.trim();
        if line.is_empty() {
            continue 'get_input;
        }

        lazy_static! {
            static ref RE_ADD_COMMAND: Regex =
                Regex::new(r"(?i:^add\s+([a-z ]+)\s+to\s+([a-z ]+)$)").unwrap();
            static ref RE_LIST_DEPARTMENT: Regex = Regex::new(r"(?i:^list\s+([a-z ]+)$)").unwrap();
            static ref RE_LIST_ALL: Regex = Regex::new(r"(?i:^list$)").unwrap();
        }

        if let Some(cap) = RE_ADD_COMMAND.captures(line) {
            return Command::AddEmployeeToDepartment {
                employee: String::from(&cap[1]),
                department: String::from(&cap[2]),
            };
        }

        if RE_LIST_ALL.is_match(line) {
            return Command::ListEmployees;
        }

        if let Some(cap) = RE_LIST_DEPARTMENT.captures(line) {
            return Command::ListEmployeesByDepartment {
                department: String::from(&cap[1]),
            };
        }

        println!("Invalid command.");
    }
}

fn add_employee(database: &mut EmployeeDatabse, employee_name: &str, department_name: &str) {
    let department = database
        .entry(String::from(department_name))
        .or_insert_with(Vec::new);
    department.push(String::from(employee_name));
    department.sort();
    department.dedup();
}

fn list_department(database: &EmployeeDatabse, department_name: &str) {
    match database.get(department_name) {
        Some(list) => println!("{:?}", &list),
        None => println!("Department not found."),
    };
}

fn list_all(database: &EmployeeDatabse) {
    for department in database {
        println!("{}: {:?}", department.0, department.1);
    }
}

fn main() {
    let mut employee_database: EmployeeDatabse = HashMap::new();

    loop {
        match get_command() {
            Command::AddEmployeeToDepartment {
                employee,
                department,
            } => add_employee(&mut employee_database, &employee, &department),
            Command::ListEmployeesByDepartment { department } => {
                list_department(&employee_database, &department)
            }
            Command::ListEmployees => list_all(&employee_database),
        };
    }
}
