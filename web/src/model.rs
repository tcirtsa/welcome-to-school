use crate::schema::*;
use diesel::prelude::*;
use diesel::Insertable;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = student)]
pub struct Student {
    pub account: String,
    pub psd: String, // 注意：实际生产中应使用哈希密码
    pub points: i32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = student)]
pub struct NewStudent {
    pub account: String,
    pub psd: String,
}

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = latlong)]
pub struct Latlong {
    pub id: String,
    pub longitude: String,
    pub latitude: String,
}
