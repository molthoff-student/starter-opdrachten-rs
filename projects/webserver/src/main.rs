use clap::Parser;
use std::io::{self, BufRead};
use tokio::{sync::oneshot, task};
use warp::Filter;

mod commands;
use commands::{Command, command_mapper};

mod launchargs;
use launchargs::LaunchArgs;

#[tokio::main]
async fn main() {
    println!("Booting up async Tokio::main...");
    let args = LaunchArgs::parse();
    let addr = args.socket_addr();
    let path = args.website_dir();

    println!(
        "Attempting to host web server...\nAddress: {:?}\nDirectory: {:?}",
        addr, path
    );

    // Sets the routing directories.
    let website = warp::fs::dir(path.join("website"));
    let script  = warp::path("script")
        .and(warp::fs::dir(path.join("script")));
    let data = warp::path("database")
        .and(warp::fs::dir(path.join("database")));
    let css = warp::path("css")
        .and(warp::fs::dir(path.join("css")));

    let routes = website.or(script).or(data).or(css);

    // Initalizes async server comms.
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    task::spawn_blocking(move || {
        let cmd_map = command_mapper();

        let stdin = io::stdin();
        let mut input_lines = stdin.lock().lines();

        println!(
            "Connection to terminal established. For more information write \"help\" and hit the \"enter\" button."
        );

        while let Some(Ok(input_line)) = input_lines.next() {
            let line = input_line.trim().to_lowercase();
            if let Some(command) = cmd_map.get(line.as_str()) {
                match command {
                    Command::Shutdown => {
                        println!("\nShutting server down...");
                        let _ = shutdown_tx.send(());
                        break;
                    }
                    Command::Status => {
                        println!("\nUnimplemented.");
                    }
                    Command::Help => {
                        println!("\nAviable commands are:\nHelp\nShutdown\nStatus");
                    }
                }
            } else {
                println!(
                    "\"{}\" is an invalid command. Try using \"Help\" to lean about the aviable commands.",
                    input_line
                );
            }
        }
    });

    warp::serve(routes)
        .bind(addr)
        .await
        .graceful(async {
            shutdown_rx.await.ok();
        })
        .run()
        .await;

    println!("Server has been shut down!");
}
