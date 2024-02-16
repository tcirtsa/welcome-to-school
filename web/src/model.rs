use crate::schema::*;
use diesel::prelude::*;
use diesel::Insertable;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = student)]
pub struct Student {
    pub id: i32,
    pub account: String,
    pub psd: String, // 注意：实际生产中应使用哈希密码
}

#[derive(Insertable, Debug)]
#[diesel(table_name = student)]
pub struct NewStudent {
    pub account: String,
    pub psd: String,
}
