use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rusty Journal",
    about = "A command line to-do app written in Rust"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    /// Use a different journal file.
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Write tasks to the journal file.
    Add {
        /// The task description text.
        #[structopt()]
        task: String,
    },
    /// Remove an entry from the journal file by position.
    Done {
        #[structopt()]
        position: usize,
    },
    /// List all tasks in the journal file.
    List,
}

// #![allow(dead_code, unused_imports)]

// use clap::{Parser, Subcommand};
// use std::path::PathBuf;

// #[derive(Parser, Debug)]
// #[command(author, version, about = "A command line to-do app written in Rust", name = "Rusty Journal", long_about = None)]
// pub struct CommandLineArgs {
//     #[command(subcommand)]
//     pub action: Action,
//     /// Use a different journal file.
//     #[arg(short, long)]
//     pub journal_file: Option<PathBuf>,
// }

// #[derive(Subcommand, Debug, Clone)]
// pub enum Action {
//     /// Write tasks to the journal file.
//     #[arg(short, long)]
//     Add {
//         /// The task description text.
//         task: String,
//     },
//     /// Remove an entry from the journal file by position.
//     Done { position: usize },
//     /// List all tasks in the journal file.
//     List,
// }

// //   use clap::{Parser, Subcommand};
// //
// //   // #[derive(Parser)]
// //   // #[command(author, version, about, long_about = None)]
// //   // struct Args {
// //   //     #[arg(short, long, default_value = "shuttle")]
// //   //     name: String,
// //   // }
// //
// //   // fn main() {
// //   //     let args = Args::parse();
// //
// //   //     println!("Hello, {}!", args.name);
// //   // }
// //
// //   #[derive(Parser)]
// //   #[command(author, version, about, long_about = None)]
// //   struct Args {
// //       #[command(subcommand)]
// //       cmd: Commands,
// //   }
// //
// //   #[derive(Subcommand, Debug, Clone)]
// //   enum Commands {
// //       Get,
// //       Set,
// //   }
// //
// //   fn main() {
// //       let args = Args::parse();
// //
// //       println!("{:?}", args);
// //   }
// //
