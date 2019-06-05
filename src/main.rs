#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate sysinfo;
#[macro_use] extern crate serde_derive;

mod metrics;
mod cors;

use metrics::Metrics;
use cors::CORS;
use rocket_contrib::json::Json;
use sysinfo::SystemExt;

#[get("/")]
fn get_metrics() -> Json<Metrics> {
    Json(compute_metrics())
} 

fn compute_metrics() -> Metrics {
    let mut system = sysinfo::System::new();
    system.refresh_all();
    
    Metrics {
        total_memory: system.get_total_memory(),
        total_used_memory: system.get_used_memory(),
        total_swap: system.get_total_swap(), 
        total_used_swap: system.get_used_swap()
    }
}

fn main() {
    rocket::ignite()
    .attach(CORS())
        .mount("/metrics", routes![get_metrics])
        .mount("/", routes![get_metrics])
    .launch();
} 