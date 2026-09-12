use crate::cli::Action;
use crate::models::Bill;
use crate::utils::db_utils::{PatchParams, WriteParams};
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::io;
use std::num::ParseFloatError;

enum IOResult {
    Completed,
    Canceled,
    Exited,
    Failed,
}

enum EditOption {
    DebtorName,
    Reason,
    AmountOwed,
    Submit,
}

impl EditOption {
    fn from(index: &i32) -> Result<Self> {
        match index {
            1 => Ok(Self::DebtorName),
            2 => Ok(Self::Reason),
            3 => Ok(Self::AmountOwed),
            4 => Ok(Self::Submit),
            _ => Err(anyhow!("Invalid option")),
        }
    }

    fn display(&self) {
        let value: &str = match self {
            Self::DebtorName => "Debtor Name:",
            Self::AmountOwed => "Amount owed:",
            Self::Reason => "Reason:",
            Self::Submit => "Submit",
        };
        println!("{value}");
    }
}

pub(crate) struct HomeParams {
    pub(crate) database_write: fn(params: WriteParams) -> Result<()>,
    pub(crate) database_read: fn() -> Result<HashMap<i32, Bill>>,
    pub(crate) database_delete: fn(id: &i32) -> Result<HashMap<i32, Bill>>,
    pub(crate) database_patch: fn(params: PatchParams) -> Result<HashMap<i32, Bill>>,
    pub(crate) database_read_one: fn(id: &i32) -> Result<Bill>,
}

struct EditParams {
    database_patch: fn(params: PatchParams) -> Result<HashMap<i32, Bill>>,
    database_read_one: fn(id: &i32) -> Result<Bill>,
}

struct DisplayEditableBillParams<'a> {
    debtor_name: &'a str,
    reason: &'a str,
    amount_owed: f64,
    show_submit: bool,
}

pub(crate) struct CLIService {}

impl CLIService {
    pub(crate) fn home(params: HomeParams) {
        loop {
            println!();
            Action::display_values();
            println!("\nEnter the option number (Eg: 1):");
            let mut action_buffer: String = String::new();

            match Self::read_io(&mut action_buffer) {
                IOResult::Canceled | IOResult::Failed => continue,
                IOResult::Exited => return,
                _ => (),
            }

            let action_id: i32 = match action_buffer.trim().parse() {
                Ok(id) => id,
                Err(_) => {
                    println!("Invalid Option");
                    continue;
                }
            };

            let action: Action = match Action::from(&action_id) {
                Ok(action) => action,
                Err(message) => {
                    println!("{message}");
                    continue;
                }
            };

            match action {
                Action::Add => match Self::add(params.database_write) {
                    IOResult::Exited => return,
                    _ => continue,
                },
                Action::View => match Self::view(params.database_read) {
                    IOResult::Exited => return,
                    _ => continue,
                },
                Action::Update => match Self::edit(EditParams {
                    database_patch: params.database_patch,
                    database_read_one: params.database_read_one,
                }) {
                    IOResult::Exited => return,
                    _ => continue,
                },
                Action::Delete => match Self::delete(params.database_delete) {
                    IOResult::Exited => return,
                    _ => continue,
                },
            }
        }
    }

    fn add(database_write: fn(params: WriteParams) -> Result<()>) -> IOResult {
        println!("Enter debtor name:");
        let mut name_buffer: String = String::new();
        match Self::read_io(&mut name_buffer) {
            IOResult::Completed => (),
            IOResult::Failed => return Self::add(database_write),
            other => return other,
        }

        let name: &str = name_buffer.trim();

        let amount_owed: f64;
        let mut amount_buffer: String = String::new();
        loop {
            println!("How much do they owe:");

            match Self::read_io(&mut amount_buffer) {
                IOResult::Completed => (),
                IOResult::Failed => continue,
                other => return other,
            }

            let amount: Result<f64, ParseFloatError> = amount_buffer.trim().parse();
            if let Ok(parsed_amount) = amount {
                amount_owed = parsed_amount;
                break;
            }
            println!("Invalid number")
        }

        let mut reason_buffer: String = String::new();

        loop {
            println!("Give a reason for this bill to differentiate it from other bills from the same debtor:");

            match Self::read_io(&mut reason_buffer) {
                IOResult::Completed => break,
                IOResult::Failed => continue,
                other => return other,
            }
        }

        let reason: &str = reason_buffer.trim();

        let params: WriteParams = WriteParams {
            debtor_name: name.to_owned(),
            reason: reason.to_owned(),
            amount_owed,
        };

        match database_write(params) {
            Ok(_) => {
                println!("Bill added");
                IOResult::Completed
            }
            Err(error) => {
                println!("{}", error);
                Self::add(database_write)
            }
        }
    }

    fn view(database_read: fn() -> Result<HashMap<i32, Bill>>) -> IOResult {
        let bills: HashMap<i32, Bill> = match database_read() {
            Ok(bills) => bills,
            Err(error) => {
                println!("{}", error);
                return IOResult::Canceled;
            }
        };

        Self::display_bills(bills);

        IOResult::Completed
    }

    fn delete(database_delete: fn(id: &i32) -> Result<HashMap<i32, Bill>>) -> IOResult {
        println!("[DELETE] Enter bill id:");

        let mut id_buffer: String = String::new();

        match Self::read_io(&mut id_buffer) {
            IOResult::Completed => (),
            IOResult::Failed => return Self::delete(database_delete),
            other => return other,
        }

        let id: i32 = match id_buffer.trim().parse() {
            Ok(id) => id,
            Err(_) => {
                println!("Invalid number");
                return Self::delete(database_delete);
            }
        };

        let updated_bills: HashMap<i32, Bill> = match database_delete(&id) {
            Ok(bills) => bills,
            Err(error) => {
                println!("{}", error);
                return Self::delete(database_delete);
            }
        };

        let mut action_buffer: String = String::new();
        let action_id: i32;

        println!("Bill deleted!\n");

        match Self::conclude_action(&mut action_buffer) {
            IOResult::Completed => action_id = action_buffer.parse().unwrap(),
            other => return other,
        }

        if action_id == 2 {
            Self::display_bills(updated_bills);
        }

        println!("Bill with id {:?} deleted.", id);
        IOResult::Completed
    }

    fn edit(params: EditParams) -> IOResult {
        println!("[EDIT] Enter bill id:");

        let mut id_buffer: String = String::new();

        match Self::read_io(&mut id_buffer) {
            IOResult::Completed => (),
            IOResult::Failed => return Self::edit(params),
            other => return other,
        }

        let id: i32 = match id_buffer.trim().parse() {
            Ok(id) => id,
            Err(_) => {
                println!("Invalid number");
                return Self::edit(params);
            }
        };

        let bill: Bill = match (params.database_read_one)(&id) {
            Ok(bill) => bill,
            Err(error) => {
                println!("{}", error);
                return Self::edit(params);
            }
        };

        let mut debtor_name: Option<String> = Some(bill.clone().debtor_name);
        let mut amount_owed: Option<f64> = Some(bill.clone().amount_owed);
        let mut reason: Option<String> = Some(bill.clone().reason);

        let mut index: i32 = 0;

        loop {
            Self::display_editable_bill(DisplayEditableBillParams {
                // Wonder how expensive cloning is 🤣💀...This looks scary how I'm cloning all over the place.
                debtor_name: &debtor_name.clone().unwrap(),
                amount_owed: amount_owed.unwrap(),
                reason: &reason.clone().unwrap(),
                show_submit: index > 0,
            });

            let mut field_buffer: String = String::new();
            let edit_option: EditOption;

            loop {
                let mut prompt: String = String::from("Enter field number");
                if index > 0 {
                    prompt.push_str(" (Enter 4 to submit)");
                }
                println!("\n{prompt}:");

                match Self::read_io(&mut field_buffer) {
                    IOResult::Completed => (),
                    IOResult::Failed => continue,
                    other => return other,
                }

                let field_number: i32 = match field_buffer.trim().parse() {
                    Ok(number) => number,
                    Err(_) => {
                        println!("Invalid number");
                        continue;
                    }
                };

                let mut upper_bound: i32 = 4;
                if index == 0 {
                    upper_bound = 3;
                }

                if field_number >= 1 && field_number <= upper_bound {
                    edit_option = EditOption::from(&field_number).unwrap();
                    break;
                }
                println!("Invalid option!");
            }

            if let EditOption::Submit = edit_option {
                break;
            }

            let mut new_value_buffer: String = String::new();

            loop {
                edit_option.display();
                match (Self::read_io(&mut new_value_buffer), &edit_option) {
                    (IOResult::Completed, EditOption::DebtorName) => {
                        debtor_name = Some(new_value_buffer.trim().to_owned())
                    }
                    (IOResult::Completed, EditOption::Reason) => {
                        reason = Some(new_value_buffer.trim().to_owned())
                    }
                    (IOResult::Completed, EditOption::AmountOwed) => {
                        let amount: f64 = match new_value_buffer.trim().parse() {
                            Ok(value) => value,
                            Err(_) => {
                                println!("Invalid number");
                                continue;
                            }
                        };
                        amount_owed = Some(amount);
                    }
                    (IOResult::Failed, _) => continue,
                    (other_result, _) => return other_result,
                }
                break;
            }
            index += 1;
        }
        let new_debtor_name: String = debtor_name.unwrap();
        let new_amount_owed: f64 = amount_owed.unwrap();
        let new_reason: String = reason.unwrap();

        let debtor_name_changed: bool = new_debtor_name != bill.debtor_name;
        let amount_owed_changed: bool = new_amount_owed != bill.amount_owed;
        let reason_changed: bool = new_reason != bill.reason;

        let patch_params: PatchParams = PatchParams {
            id: bill.id,
            debtor_name: if debtor_name_changed {
                Some(new_debtor_name)
            } else {
                None
            },
            amount_owed: if amount_owed_changed {
                Some(new_amount_owed)
            } else {
                None
            },
            reason: if reason_changed {
                Some(new_reason)
            } else {
                None
            },
        };

        let updated_bills: HashMap<i32, Bill> = match (params.database_patch)(patch_params) {
            Ok(bills) => bills,
            Err(error) => {
                println!("{}", error);
                return Self::edit(params);
            }
        };

        let mut action_buffer: String = String::new();
        let action_id: i32;

        println!("Bill Updated!\n");

        match Self::conclude_action(&mut action_buffer) {
            IOResult::Completed => action_id = action_buffer.parse().unwrap(),
            other => return other,
        }

        if action_id == 2 {
            Self::display_bills(updated_bills);
        }

        println!("Bill updated");
        IOResult::Completed
    }

    fn display_editable_bill(params: DisplayEditableBillParams) {
        if params.show_submit {
            Self::display_preview_divider();
        } else {
            Self::display_divider();
        }

        for i in 1..=3 {
            let option: EditOption = EditOption::from(&i).unwrap();
            match option {
                EditOption::DebtorName => println!("{}. Debtor Name: {}", i, params.debtor_name),
                EditOption::Reason => println!("{}. Reason: {}", i, params.reason),
                EditOption::AmountOwed => println!("{}. Amount owed: {}", i, params.amount_owed),
                _ => (),
            }
        }

        if params.show_submit {
            println!("4. Submit");
        }
        Self::display_divider();
    }

    fn conclude_action(id_buffer: &mut String) -> IOResult {
        let mut action_buffer: String = String::new();

        loop {
            println!("1. Done");
            println!("2. View Bills");
            println!();
            println!("Enter option number:");
            match Self::read_io(&mut action_buffer) {
                IOResult::Completed => (),
                IOResult::Failed => continue,
                other => return other,
            }

            let trimmed_value: &str = action_buffer.trim();

            if trimmed_value == "1" || trimmed_value == "2" {
                id_buffer.push_str(trimmed_value);
                return IOResult::Completed;
            }

            println!("Invalid number!");
        }
    }

    fn display_divider() {
        println!("----------------------------------------------------------------------");
    }

    fn display_preview_divider() {
        println!("-------------------------------PREVIEW-------------------------------");
    }

    fn display_bills(bills: HashMap<i32, Bill>) {
        if bills.is_empty() {
            return println!("No bills. Create one to see them here.");
        }
        let mut index: i32 = 0;
        for bill in bills.values() {
            if index == 0 {
                Self::display_divider();
            }
            bill.display();
            Self::display_divider();
            index += 1;
        }
    }

    fn read_io(buffer: &mut String) -> IOResult {
        buffer.clear();
        match io::stdin().read_line(buffer) {
            Ok(_) => (),
            Err(error) => {
                println!("{}", error);
                return IOResult::Failed;
            }
        }

        if buffer.trim().eq_ignore_ascii_case("cancel") {
            return IOResult::Canceled;
        }
        if buffer.trim().eq_ignore_ascii_case("exit") {
            return IOResult::Exited;
        }

        IOResult::Completed
    }
}
