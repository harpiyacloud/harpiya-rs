mod controller;
mod domain;
mod extension;
mod logic;
mod middleware;
mod model;
mod router;
mod schedule;
mod service;

use harpiya_rs::prelude::*;

fn main() {
    harpiya_rs::Cluster::boot()
        .register(router::routes())
        .register_debug(router::debug_routes())
        .spawn(schedule::job_scheduler())
        .run_with(schedule::async_job_scheduler())
}
