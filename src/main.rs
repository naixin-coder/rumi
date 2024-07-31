use std::{env, path::Path};

use actix_files as fs;
use actix_web::{App, HttpServer};
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

async fn serve(path: String, port: u16) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new().service(fs::Files::new("/", Path::new(&path)).index_file("index.html"))
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
            let _ = serve(new_path, port).await;
        }
    }
}
