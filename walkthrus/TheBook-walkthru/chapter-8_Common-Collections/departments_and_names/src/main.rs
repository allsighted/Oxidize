/*  
 * This program demonstrates how to use a HashMap in Rust to store and manage a collection of employees
 * grouped by their respective departments. It allows for adding new employees to a department and 
 * retrieving the list of employees in a specific department.

 * REPL:
 * - Be able to create a new deparment `> create department <department_name>`
 * - Be able to add an employee to a department `> add employee <employee_name> to <department_name>`
 * - Be able to get a list of employees in a department `> get employees in <department_name>`
 * - Be able to remove an employee from a department `> remove employee <employee_name> from <department_name>`
 * - Be able to remove a department `> remove department <department_name>`
 * - Be able to get a list of all departments `> get all departments`
 * - Be able to get a list of all employees `> get all employees`
 * - Be able to get a list of all employees in all departments sorted by department `> get all employees by department`
*/

use std::collections::HashMap;

struct Company {
    departments: HashMap<String, Vec<String>>,
}

impl Company {
    fn new() -> Self {
        Company {
            departments: HashMap::new(),
        }
    }

    fn remove_department(&mut self, department_name: &str) {
        if self.departments.remove(department_name).is_none() {
            println!("Department {} does not exist.", department_name);
        }
    }

    fn remove_employee(&mut self, employee_name: &str, department_name: &str) {
        if let Some(employees) = self.departments.get_mut(department_name) {
            if let Some(pos) = employees.iter().position(|x| x == employee_name) {
                employees.remove(pos);
            } else {
                println!("Employee {} does not exist in department {}.", employee_name, department_name);
            }
        } else {
            println!("Department {} does not exist.", department_name);
        }
    }

    fn add_department(&mut self, department_name: String) {
        if self.departments.contains_key(&department_name) {
            println!("Department {} already exists.", department_name);
        } else {
            self.departments.insert(department_name, Vec::new());
        }
    }

    fn add_employee(&mut self, employee_name: String, department_name: &str) {
        if let Some(employees) = self.departments.get_mut(department_name) {
            if !employees.contains(&employee_name) {
                employees.push(employee_name);
                employees.sort();
            } else {
                println!("Employee already exists in department {}.", department_name);
            }
        } else {
            println!("Department {} does not exist.", department_name);
        }
    }

    fn get_employees(&self, department_name: &str) -> Option<&Vec<String>> {
        self.departments.get(department_name)
    }

    fn get_all_employees(&self) -> Vec<String> {
        let mut all_employees: Vec<String> = Vec::new();
        for employees in self.departments.values() {
            all_employees.extend(employees.clone());
        }
        all_employees.sort();
        all_employees.dedup();
        all_employees
    }

    fn get_all_employees_by_department(&self) -> HashMap<String, Vec<String>> {
        let mut all_employees_by_department: HashMap<String, Vec<String>> = HashMap::new();
        for (department, employees) in &self.departments {
            all_employees_by_department.insert(department.clone(), employees.clone());
        }
        all_employees_by_department
    }

    fn get_all_departments(&self) -> Vec<String> {
        self.departments.keys().cloned().collect()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut company = Company::new();

    run_repl(&mut company);
    Ok(())
}

fn run_repl(company: &mut Company) {
    use std::io::{self, Write};

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            break;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        match parts.as_slice() {
            ["create", "department", department_name] => {
                company.add_department(department_name.to_string());
            }
            ["add", "employee", employee_name, "to", department_name] => {
                company.add_employee(employee_name.to_string(), department_name);
            }
            ["get", "employees", "in", department_name] => {
                if let Some(employees) = company.get_employees(department_name) {
                    println!("Employees in {}: {:?}", department_name, employees);
                } else {
                    println!("Department {} does not exist.", department_name);
                }
            }
            ["remove", "employee", employee_name, "from", department_name] => {
                company.remove_employee(employee_name, department_name);
            }
            ["remove", "department", department_name] => {
                company.remove_department(department_name);
            }
            ["get", "all", "departments"] => {
                let departments = company.get_all_departments();
                println!("Departments: {:?}", departments);
            }
            ["get", "all", "employees"] => {
                let all_employees = company.get_all_employees();
                println!("All Employees: {:?}", all_employees);
            }
            ["get", "all", "employees", "by", "department"] => {
                let all_employees_by_department = company.get_all_employees_by_department();
                for (department, employees) in all_employees_by_department {
                    println!("{}: {:?}", department, employees);
                }
            }
            ["exit"] => {
                println!("Exiting...");
                break;
            }
            ["help"] => {
                println!("Available commands:");
                println!("1. create department <department_name>");
                println!("2. add employee <employee_name> to <department_name>");
                println!("3. get employees in <department_name>");
                println!("4. remove employee <employee_name> from <department_name>");
                println!("5. remove department <department_name>");
                println!("6. get all departments");
                println!("7. get all employees");
                println!("8. get all employees by department");
                println!("9. exit");
                println!("10. help");
            }
            _ => println!("Invalid command. Type 'help' for a list of commands."),
        }
    }
}


