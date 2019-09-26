
use diesel::query_builder::functions::sql_query;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use diesel::sql_types::Text;

use diesel::result::Error as err;

use crate::schema;
use crate::models::*;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn resolve(q_method: String, q_host: String, q_path: String) {
    // use schema::apps::dsl::*;
    // use schema::routes::dsl::*;
    // use schema::views::dsl::*;
    use schema::*;

    println!("Method: {} host {} path {}", q_method, q_host, q_path);
    let host_lower = q_host.to_lowercase();

    let pg_conn = establish_connection();

    let app_filter = apps::table.filter(apps::domain.eq(host_lower)).first::<App>(&pg_conn).unwrap();
    // let route_filter = Route::belonging_to(&app_filter).filter(pattern.eq(q_path)).first::<Route>(&pg_conn).unwrap();
    // let results = View::belonging_to(&app_filter)
    //     .inner_join(
    //         routes::table.on(
    //             views::id.eq(routes::view_id).and(
    //                 routes::pattern.eq(q_path)
    //             )
    //         )
    //     )
    //     .limit(1)
    //     //.load::<View>(&pg_conn)
    //     .expect("Error loading apps");
    let results: Vec<(View, Route)> = View::belonging_to(&app_filter).inner_join(
            routes::table.on(
                views::id.eq(routes::view_id).and(
                    routes::pattern.eq(q_path)
                )
            )
    ).limit(1)
    .load(&pg_conn)
    .expect("Error loading apps");

    // let views: Result<Vec<View>, err> = sql_query("select * from views 
    // inner join routes on views.id = routes.view_id 
    // inner join apps on routes.app_id = apps.id 
    // where apps.\"domain\" = ? and routes.pattern = ?;")
    // .bind::<Text, _>(host_lower)
    // .bind::<Text, _>(q_path)
    // .get_results(&pg_conn);

    println!("PG result {:?}", results);
}
