#![allow(unused)]

use tera::Tera;
use tide_tera::prelude::*;

mod core;

use crate::core::{constants::*, functions::*, structs::*};

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();

    let mut ctx = Context::new(TEMPLATE_DIR);
    ctx.details.insert("title", APP_TITLE);
    ctx.details.insert("version", APP_VERSION);

    let mut app = tide::with_state(ctx);

    app.at("/static").serve_dir("static");

    app.at("/")
        .get(|mut req: tide::Request<Context>| async move {
            req.state()
                .tera
                .render_response("index.html", &req.state().get())
        });

    app.listen("0.0.0.0:80").await?;

    Ok(())
}
