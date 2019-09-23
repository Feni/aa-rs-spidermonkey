use std::time::SystemTime;
use crate::schema::*;

#[derive(Identifiable, Queryable, PartialEq, Debug)]
//#[table_name = "apps"]
pub struct App {
    pub id: i32,
    pub name: String,
    pub domain: String,
    pub environment: u16,
    
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Identifiable, Queryable, PartialEq, Debug, Associations)]
#[belongs_to(App)]
pub struct View {
    pub id: i32,
    pub app_id: i32,
    pub name: String,
    pub kind: i16,

    pub content_url: Option<String>,
    pub content: Option<String>,
    
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Identifiable, Queryable, PartialEq, Debug, Associations)]
pub struct Route {
    pub id: i32,
    pub app_id: i32, 
    pub name: Option<String>,
    pub raw: String,
    pub pattern: String,

    pub method_get: bool, 
    pub method_post: bool, 
    pub extra_methods: Option<Vec<u16>>,

    pub view_id: i32,
    
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

