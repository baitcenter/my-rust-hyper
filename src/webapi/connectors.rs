#[cfg(not(test))]
use super::collections;
#[cfg(test)]
use super::tests::fakes;
use super::{entities, settings, errors};
#[cfg(all(not(test), feature = "mysql"))]
use sqlx::MySqlPool;
#[cfg(all(not(test), feature = "postgres"))]
use sqlx::PgPool;
use std::collections::HashMap;
#[cfg(not(test))]
use std::sync::Arc;

pub type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

pub const PROTO_HTTP: &str = "http";
// pub const PROTO_MQ: &str = "mq";

#[cfg(not(test))]
pub struct ExpHelper;

#[cfg(not(test))]
impl ExpHelper {
    fn new() -> &'static ExpHelper {
        &ExpHelper {}
    }

    pub fn get_int_ids_as_exp(&self, ids: &Vec<i32>) -> String {
        let mut result: String = String::with_capacity(100);
        for item in ids {
            if result.len() != 0 {
                result.push(',');
            }
            result.push_str(&item.to_string());
        }
        result
    }

    pub fn get_str_ids_exp(&self, ids: &Vec<String>) -> String {
        let mut result: String = String::with_capacity(100);
        for item in ids {
            if result.len() != 0 {
                result.push(',');
            }
            result.push_str(&format!("'{}'", item.to_string()));
        }
        result
    }

    pub fn get_select_int_exp(&self, table: &str, field: &str, ids: &Vec<i32>) -> String {
        format!(
            "SELECT * FROM {} WHERE {} IN ({})",
            table,
            field,
            self.get_int_ids_as_exp(ids)
        )
    }

    pub fn get_delete_int_exp(&self, table: &str, field: &str, ids: &Vec<i32>) -> String {
        format!(
            "DELETE FROM {} WHERE {} IN ({})",
            table,
            field,
            self.get_int_ids_as_exp(ids)
        )
    }

    pub fn get_select_str_exp(&self, table: &str, field: &str, ids: &Vec<String>) -> String {
        format!(
            "SELECT * FROM {} WHERE {} IN ({})",
            table,
            field,
            self.get_str_ids_exp(ids)
        )
    }

    pub fn get_delete_str_exp(&self, table: &str, field: &str, ids: &Vec<String>) -> String {
        format!(
            "DELETE FROM {} WHERE {} IN ({})",
            table,
            field,
            self.get_str_ids_exp(ids)
        )
    }
}

pub struct DataConnector {
    pub error: HashMap<String, String>,
    #[cfg(not(test))]
    pub usr: collections::usr::UsrCollection,
    #[cfg(test)]
    pub usr: fakes::usr::UsrCollection,
    #[cfg(not(test))]
    pub car: collections::car::CarCollection,
    #[cfg(test)]
    pub car: fakes::car::CarCollection,
    #[cfg(not(test))]
    pub route: collections::route::RouteCollection,
    #[cfg(test)]
    pub route: fakes::route::RouteCollection,
}

impl DataConnector {
    pub async fn new(
        _error: Option<HashMap<String, String>>,
        _db: &settings::Database
    ) -> Result<DataConnector> {
        #[cfg(not(test))]
        let _exp_helper: &'static ExpHelper = &ExpHelper::new();
        #[cfg(all(not(test), feature = "postgres"))]
        let dp = SqlDbProvider::new(&_db.connection_string).await?;
        #[cfg(all(not(test), feature = "mysql"))]
        let dp = SqlDbProvider::new(&_db.connection_string).await?;
        let mut error = HashMap::<String, String>::new();
        if _error.is_some() {
            error.extend(_error.unwrap());
        }
        #[cfg(not(test))]
        error.extend(DataConnector::errors_as_hashmap(dp.get_errors().await?));
        #[cfg(not(test))]
        let _dp_arc = Arc::new(dp);
        Ok(DataConnector {
            error: error,
            #[cfg(not(test))]
            usr: collections::usr::UsrCollection::new(_dp_arc.clone(), &_exp_helper),
            #[cfg(test)]
            usr: fakes::usr::UsrCollection::new(),
            #[cfg(not(test))]
            car: collections::car::CarCollection::new(_dp_arc.clone(), &_exp_helper),
            #[cfg(test)]
            car: fakes::car::CarCollection::new(),
            #[cfg(not(test))]
            route: collections::route::RouteCollection::new(_dp_arc.clone(), &_exp_helper),
            #[cfg(test)]
            route: fakes::route::RouteCollection::new(),
        })
    }

    #[cfg(not(test))]
    fn errors_as_hashmap(items: Vec<entities::error::Error>) -> HashMap<String, String> {
        let mut error = HashMap::<String, String>::new();
        for item in items {
            error.insert(item.error_code, item.error_name);
        }
        error
    }
}

#[cfg(not(test))]
pub struct SqlDbProvider {
    #[cfg(feature = "postgres")]
    pub pool: Arc<PgPool>,
    #[cfg(feature = "mysql")]
    pub pool: Arc<MySqlPool>,
}

#[cfg(not(test))]
impl SqlDbProvider {
    pub async fn new(connection_string: &String) -> Result<SqlDbProvider> {
        debug!("connection string {}", connection_string);
        #[cfg(feature = "postgres")]
        let pool = PgPool::new(&connection_string).await.unwrap();
        #[cfg(feature = "mysql")]
        let pool = MySqlPool::new(&connection_string).await.unwrap();
        Ok(SqlDbProvider {
            pool: Arc::new(pool),
        })
    }

    pub async fn get_errors(&self) -> Result<Vec<entities::error::Error>> {
        Ok(
            vec![entities::error::Error{
                error_code: errors::ErrorCode::DatabaseError.to_string(),
                error_name: "Database error".to_string()
            },
            entities::error::Error{
                error_code: errors::ErrorCode::NotFoundError.to_string(),
                error_name: "Some items with specified id is not found".to_string()
            }]
        )
    }
}
