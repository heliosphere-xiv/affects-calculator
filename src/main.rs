use clap::Parser;
use ironworks::{
    Ironworks,
    excel::{Excel, Language},
    sqpack::{Install, SqPack},
};

// use ironworks_sheets::{for_type, sheet};
use crate::{
    cli::CliArguments,
    schema::{Item, MetadataProvider},
};

#[macro_use]
mod macros;

mod cli;
mod parser;
mod schema;

fn main() {
    // let f = File::open("CurrentPathList").unwrap();
    // let reader = BufReader::new(f);

    // let mut missing = BufWriter::new(File::create("/home/anna/missing.txt").unwrap());

    // let mut success = 0;
    // let mut total = 0;
    // const BAD_EXTS: &[&str] = &[".luab", ".exd", ".exh", ".lgb", ".sgb"];
    // for line in reader.lines() {
    //     let line = line.unwrap();
    //     if line.starts_with(|c: char| c.is_ascii_uppercase() || c.is_numeric())
    //         || BAD_EXTS.iter().any(|ext| line.ends_with(ext))
    //     {
    //         continue;
    //     }

    //     total += 1;
    //     if crate::parser::GamePath::parse(&line).is_ok() {
    //         success += 1;
    //     } else {
    //         missing.write_all(line.as_bytes()).unwrap();
    //         missing.write_all(b"\n").unwrap();
    //     }
    // }

    // let pct = success as f32 / total as f32 * 100.0;
    // println!("{success}/{total} ({pct}%)");

    let args = CliArguments::parse();
    let ironworks = Ironworks::new().with_resource(SqPack::new(Install::at(&args.game_path)));
    let excel = Excel::new(ironworks).with_default_language(Language::English);
    let items = excel.sheet(MetadataProvider::<Item>::for_sheet()).unwrap();
    for item in items {
        let name = item.name.format().unwrap();
        if name.is_empty() {
            continue;
        }

        println!("{name}: {:x} / {:x}", item.model_main, item.model_sub);
    }
}
