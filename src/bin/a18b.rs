// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

use std::io;

#[derive(Clone)]
enum Role {
    Maintenance,
    Marketer,
    Manager,
    LineSupervisor,
    Kitchen,
    AssemblyTechnician,
}

enum EmployeeStatus {
    Terminated,
    Active,
    Suspended { end_date: String },
}

struct Employee {
    id: i32,
    name: String,
    role: Role,
    status: EmployeeStatus,
}

fn access_building(employee: &Employee) -> Result<bool, String> {
    match &employee.status {
        EmployeeStatus::Active => (),
        EmployeeStatus::Suspended { end_date } => {
            return Err(format!("You cannot access the building until {}", end_date))
        }
        EmployeeStatus::Terminated => return Err("You cannot access the building".to_owned()),
    }

    match employee.role {
        Role::Maintenance | Role::Manager | Role::Marketer => Ok(true),
        _ => Err("You do not have access to this building".to_owned()),
    }
}

fn display_access(employee: &Employee) -> Result<(), String> {
    println!("Hello {}", employee.name);
    let access_granted: bool = access_building(employee)?;
    if access_granted {
        println!("Access Granted.");
        return Ok(());
    }

    Err("Access denied.".to_owned())
}

fn generate_all_possible_combinations() -> Vec<Employee> {
    let roles: [Role; 6] = [
        Role::AssemblyTechnician,
        Role::Kitchen,
        Role::Manager,
        Role::Marketer,
        Role::Maintenance,
        Role::LineSupervisor,
    ];

    let statuses: fn() -> [EmployeeStatus; 3] = || {
        [
            EmployeeStatus::Suspended {
                end_date: "2026-09-07".to_owned(),
            },
            EmployeeStatus::Active,
            EmployeeStatus::Terminated,
        ]
    };

    let mut employees: Vec<Employee> = Vec::new();
    let mut id: i32 = 1;

    for role in roles {
        for status in statuses() {
            let employee = Employee {
                id,
                name: format!("Employee_{}", id),
                role: role.clone(),
                status,
            };
            employees.push(employee);
            id += 1
        }
    }

    employees
}

fn get_employee(id: i32) -> Result<Employee, String> {
    let employees: Vec<Employee> = generate_all_possible_combinations();
    for employee in employees {
        if employee.id == id {
            return Ok(employee);
        }
    }
    Err("Employee not found!".to_owned())
}

fn main() {
    println!("Enter your id:");

    let mut employee_id_input: String = String::new();

    io::stdin()
        .read_line(&mut employee_id_input)
        .expect("Something went wrong.");

    let employee_id: i32 = match employee_id_input.trim().parse() {
        Ok(id) => id,
        Err(_) => {
            println!("Invalid id");
            return;
        }
    };

    let employee: Employee = match get_employee(employee_id) {
        Ok(employee) => employee,
        Err(message) => {
            println!("{message}");
            return;
        }
    };

    match display_access(&employee) {
        Ok(_) => (),
        Err(message) => println!("{message}"),
    }
}
