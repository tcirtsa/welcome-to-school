use crate::schema::*;
use diesel::prelude::*;
use diesel::Insertable;
use diesel::Queryable;

#[derive(Queryable, Insertable, AsChangeset)]
#[diesel(table_name = student)]
pub struct Student {
    pub id: i32,
    pub account: String,
    pub psd: String, // 注意：实际生产中应使用哈希密码
    pub points:i32
}

#[derive(Insertable, Debug)]
#[diesel(table_name = student)]
pub struct NewStudent {
    pub account: String,
    pub psd: String,
}
