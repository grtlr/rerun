//! This example shows how to add custom Space Views to the Rerun Viewer.

use re_viewer::external::{re_log, re_memory};

mod error;
mod graph;
mod view;
mod layout;
mod ui;
mod visualizers;
mod types;

// By using `re_memory::AccountingAllocator` Rerun can keep track of exactly how much memory it is using,
// and prune the data store when it goes above a certain limit.
// By using `mimalloc` we get faster allocations.
#[global_allocator]
static GLOBAL: re_memory::AccountingAllocator<mimalloc::MiMalloc> =
    re_memory::AccountingAllocator::new(mimalloc::MiMalloc);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Direct calls using the `log` crate to stderr. Control with `RUST_LOG=debug` etc.
    re_log::setup_logging();

    // Install handlers for panics and crashes that prints to stderr and send
    // them to Rerun analytics (if the `analytics` feature is on in `Cargo.toml`).
    re_crash_handler::install_crash_handlers(re_viewer::build_info());

    // Listen for TCP connections from Rerun's logging SDKs.
    // There are other ways of "feeding" the viewer though - all you need is a `re_smart_channel::Receiver`.
    let rx = re_sdk_comms::serve(
        "0.0.0.0",
        re_sdk_comms::DEFAULT_SERVER_PORT,
        Default::default(),
    )?;

    let startup_options = re_viewer::StartupOptions::default();

    // This is used for analytics, if the `analytics` feature is on in `Cargo.toml`
    let app_env = re_viewer::AppEnvironment::Custom("Rerun Graph Viewer".to_owned());

    println!(
        "This example starts a graph viewer that is ready to accept data… you have to give it some!"
    );
    println!("Try for example to run: `cargo run -p node_link_graph -- --connect` in another terminal instance.");

    re_viewer::run_native_app(
        Box::new(move |cc| {
            let mut app = re_viewer::App::new(
                re_viewer::build_info(),
                &app_env,
                startup_options,
                cc.egui_ctx.clone(),
                cc.storage,
            );
            app.add_receiver(rx);

            // Register the custom space view
            app.add_space_view_class::<graph_space_view::GraphSpaceView>()
                .unwrap();

            Box::new(app)
        }),
        None,
    )?;

    Ok(())
}
