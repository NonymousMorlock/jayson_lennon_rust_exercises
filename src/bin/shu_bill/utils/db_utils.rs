use crate::models::Bill;
use anyhow::{anyhow, bail, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub(crate) struct DBUtils {}

struct WriteJsonFileParams {
    file_path: String,
    content: String,
}

pub(crate) struct PatchParams {
    pub(crate) id: i32,
    pub(crate) debtor_name: Option<String>,
    pub(crate) amount_owed: Option<f64>,
    pub(crate) reason: Option<String>,
}

pub(crate) struct WriteParams {
    pub(crate) debtor_name: String,
    pub(crate) reason: String,
    pub(crate) amount_owed: f64,
}

impl DBUtils {
    fn bill_path() -> &'static str {
        "db/bills.json"
    }

    fn id_path() -> &'static str {
        "db/id.txt"
    }

    fn get_next_id() -> Result<i32> {
        let id_path: &str = Self::id_path();
        let id_string: String = fs::read_to_string(id_path).unwrap_or_else(|_| "0".to_owned());
        let current_id: i32 = id_string.parse()?;

        Ok(current_id + 1)
    }

    fn update_id(value: &i32) -> Result<()> {
        Self::write_json_file(WriteJsonFileParams {
            file_path: Self::id_path().to_owned(),
            content: value.to_string(),
        })
    }

    pub(crate) fn write(params: WriteParams) -> Result<()> {
        let file_path = Self::bill_path();
        let mut bills: HashMap<i32, Bill> =
            Self::read_json_file(file_path).unwrap_or_else(|_| HashMap::new());

        let next_id: i32 = Self::get_next_id().expect("Something went wrong");

        let bill: Bill = Bill {
            id: next_id,
            debtor_name: params.debtor_name,
            reason: params.reason,
            amount_owed: params.amount_owed,
        };

        bills.insert(next_id, bill);

        Self::update_id(&next_id)?;

        let write_params = WriteJsonFileParams {
            file_path: file_path.to_owned(),
            content: serde_json::to_string_pretty(&bills)?,
        };
        Self::write_json_file(write_params)
    }

    pub(crate) fn read_all() -> Result<HashMap<i32, Bill>> {
        let bills: HashMap<i32, Bill> = Self::read_json_file(Self::bill_path())?;
        Ok(bills)
    }

    pub(crate) fn read(id: &i32) -> Result<Bill> {
        let bills: HashMap<i32, Bill> = Self::read_all()?;
        match bills.get(id) {
            Some(bill) => Ok(bill.clone()),
            None => Err(anyhow!("Bill not found!")),
        }
    }

    pub(crate) fn patch(params: PatchParams) -> Result<HashMap<i32, Bill>> {
        let mut bills: HashMap<i32, Bill> = Self::read_all()?;

        if !bills.contains_key(&params.id) {
            bail!("Bill not found!");
        };

        bills.entry(params.id).and_modify(|bill| {
            if let Some(name) = params.debtor_name {
                bill.debtor_name = name;
            }

            if let Some(amount) = params.amount_owed {
                bill.amount_owed = amount;
            }

            if let Some(reason) = params.reason {
                bill.reason = reason;
            }
        });

        let write_params: WriteJsonFileParams = WriteJsonFileParams {
            file_path: Self::bill_path().to_owned(),
            content: serde_json::to_string_pretty(&bills)?
        };

        Self::write_json_file(write_params)?;

        Ok(bills)
    }

    pub(crate) fn delete(id: &i32) -> Result<HashMap<i32, Bill>> {
        let mut bills: HashMap<i32, Bill> = Self::read_all()?;

        if !bills.contains_key(id) {
            bail!("Bill not found!");
        }

        bills.remove(id);
        Self::write_json_file(WriteJsonFileParams {
            file_path: Self::bill_path().to_owned(),
            content: serde_json::to_string_pretty(&bills)?,
        })?;

        Ok(bills)
    }

    fn write_json_file(params: WriteJsonFileParams) -> Result<()> {
        let path: &Path = Path::new(params.file_path.as_str());

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(path, params.content)?;

        Ok(())
    }

    fn read_json_file(file_path: &str) -> Result<HashMap<i32, Bill>> {
        let path: &Path = Path::new(file_path);

        if !path.is_file() {
            return Ok(HashMap::new());
        }

        let file_content: String = fs::read_to_string(file_path)?;
        let bills: HashMap<i32, Bill> = serde_json::from_str(&file_content)?;
        Ok(bills)
    }
}
