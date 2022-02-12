use sqlx::any::AnyRow;
use sqlx::any::AnyPool;
use sqlx::any::AnyPoolOptions;
use futures::TryStreamExt;
use sqlx::Row;

use std::collections::HashMap;
use std::sync::RwLock;

use super::data::Base;

pub struct Pool {
    map: RwLock<HashMap<String, AnyPool>>,
}

impl Pool {
    pub fn new() -> Pool {
        Pool {
            map: RwLock::new(HashMap::new()),
        }
    }

    pub async fn run(&self, base: &Base, sql_source: &str) -> Result<u64, sqlx::Error> {
        self.check_new(base)?;
        let map_read = self.map.read().unwrap();
        let pool = map_read.get(&base.name).unwrap();
        let mut link = pool.acquire().await?; 
        let result = sqlx::query(sql_source).execute(&mut link).await?;
        Ok(result.rows_affected())
    }

    pub async fn ask(&self, base: &Base, sql_source: &str) -> Result<String, sqlx::Error> {
        self.check_new(base)?;
        let map_read = self.map.read().unwrap();
        let pool = map_read.get(&base.name).unwrap();
        let mut link = pool.acquire().await?;
        let mut result = sqlx::query(sql_source).fetch(&mut link);
        let mut body = String::new();
        while let Some(row) = result.try_next().await? {
            let columns = row.columns();
            for i in 0..columns.len() {
                let value = format_value_of_row_in_column(&row, i)?;
                if i > 0 {
                    body.push(',');
                }
                body.push_str(&value);
            }
            body.push('\n');
        }
        Ok(body)
    }

    fn check_new(&self, base: &Base) -> Result<(), sqlx::Error> {
        let need_new = {
            let map_read = self.map.read().unwrap();
            !map_read.contains_key(&base.name)
        };
        if need_new {
            let new_pool = AnyPoolOptions::new()
                .max_connections(12)
                .connect_lazy(&base.info)?;
            let mut map_write = self.map.write().unwrap();
            map_write.insert(String::from(&base.name), new_pool);
        }
        Ok(())
    }
}

fn format_value_of_row_in_column(row: &AnyRow, column: usize) -> Result<String, sqlx::Error> {
    let mut got: Option<String> = None;
    let mut err: Option<sqlx::Error> = None;
    let value: Result<String, _> = row.try_get(column);
    if let Ok(value) = value {
        got = Some(value);
    } else {
        err = value.err();
    }
    if let Some(value) = got {
        return Ok(get_column_value_for_csv(value));
    } else {
        return Err(err.unwrap());
    }
}

fn get_column_value_for_csv(column_value: String) -> String {
    let mut result = column_value
        .replace('"', "\"\"")
        .replace('\\', "\\\\")
        .replace("\r", "\\r")
        .replace("\n", "\\n")
        .replace("\t", "\\t");
    if result.contains('"') || result.contains(",") {
        result.insert(0, '"');
        result.push('"');
    }
    result
}
