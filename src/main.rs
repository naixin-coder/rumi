// use actix_files as fs;
// use actix_web::{App, HttpServer};
// use clap::{Parser, Subcommand};
// use std::path::Path;

// // #[derive(Parser)] // requires `derive` feature
// // #[command(version, about, long_about = None)]
// // struct Cli {
// //     // #[arg(short, long)]
// //     // name: Option<String>,
// //     // #[command(subcommand)]
// //     #[command(subcommand)]
// //     command: Commands,
// // }

// // #[derive(Subcommand)]
// // enum Commands {
// //     Create { name: Option<String> },
// // }

// #[derive(Parser)] // requires `derive` feature
// #[command(version, about, long_about = None)]
// struct Cli {
//     // #[arg(short, long)]
//     // name: Option<String>,
//     // #[command(subcommand)]
//     #[command(subcommand)]
//     command: Commands,
// }

// #[derive(Subcommand)]
// enum Commands {
//     Serve { name: Option<String> },
// }

// #[actix_web::main]
// fn main() {
//     // println!(
//     //     "{:?},{:?}",
//     //     Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static")).to_str(),
//     //     std::env::current_dir()
//     // );

//     // HttpServer::new(|| {
//     //     App::new().service(
//     //         fs::Files::new(
//     //             "/",
//     //             Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
//     //         )
//     //         .index_file("index.html"),
//     //     )
//     // })
//     // .bind(("127.0.0.1", 8080))?
//     // .run()
//     // .await

//     let cli = Cli::parse();
//     println!("name: {:?}", cli.name);
//     if let Some(name) = cli.name.as_deref() {
//         println!("Value1 for name: {name}");
//     }

//     match &cli.command {
//         Commands::Serve { name } => {
//             println!("Command for name is Create: {:?}", name);
//         }
//     }
// }

// // 命令  rumi serve --path=dist --port=3001

// // 运行

// // Cli 命令 serve

use std::{
    env,
    path::Path,
    sync::{Arc, Mutex},
    thread::Thread,
};

use actix_files as fs;
use actix_web::{web, App, HttpServer};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)] //默认的命令
struct Cli {
    #[arg(long)]
    path: Option<String>,
    #[arg(short, long)]
    port: Option<String>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // static 和 port参数
    Serve {
        #[arg(long)]
        path: String,
        #[arg(short, long)]
        port: u16,
    },
}
//

async fn serve(path: &'static str, port: u16) -> std::io::Result<()> {
    let current_dir = env::current_dir().unwrap();

    // let ps = path;
    // let ps = path.clone();

    // HttpServer::new(|| {
    //     App::new().service(
    //         web::resource("/dist/{file:.*}").route(web::get().to(|| async {
    //             fs::NamedFile::open("dist/{file}")
    //                 .map_err(|_| actix_web::error::ErrorNotFound("Not Found"))
    //         })),
    //     )
    // })
    // .bind(("127.0.0.1", port))?
    // .run()
    // .await

    HttpServer::new(move || {
        App::new().service(fs::Files::new("/", Path::new(path)).index_file("index.html"))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

#[actix_web::main]
async fn main() {
    // 获取命名参数

    let cli: Cli = Cli::parse();

    // print!("{:?},{:?}", cli.path, cli.port);

    match cli.command {
        Commands::Serve { path, port } => {
            let current_dir = env::current_dir().unwrap();
            println!("Command for path is Serve: {:?}", port);
            let new_path = format!("{}/{}", current_dir.to_str().unwrap(), path);
            println!("Command for new path is Serve: {:?}", new_path.as_str());
            let _ = serve(&new_path, port).await;
        }
    }
}
