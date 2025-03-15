use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufReader, BufWriter, Write},
    sync::Arc,
    time::Instant,
};

use clap::Parser;
use ironworks::{
    Ironworks,
    excel::{Excel, Language},
    sqpack::{Install, SqPack},
};

use crate::{
    analysers::GeneratorContext,
    cli::CliArguments,
    containers::{Affects, BNpcContainer, GraphqlContainer},
};

mod analysers;
mod cli;
mod containers;

fn time(name: &str, mut f: impl FnMut()) {
    print!("{name}...");
    std::io::stdout().flush().ok();
    let start = Instant::now();
    f();
    println!(" {}ms", start.elapsed().as_millis());
}

fn main() {
    let args = CliArguments::parse();

    // handle bnpcs
    let bnpcs = if args.bnpc_path == "download" {
        println!("downloading bnpc data...");
        let bnpcs = ureq::post("https://gubal.ffxivteamcraft.com/graphql")
            .send_json(serde_json::json!({
                "query": "query { bnpc { bnpcBase, bnpcName } }",
            }))
            .unwrap()
            .body_mut()
            .read_json::<GraphqlContainer<BNpcContainer>>()
            .unwrap()
            .data;

        let mut bnpcs_file = BufWriter::new(File::create("bnpcs.json").unwrap());
        serde_json::to_writer_pretty(&mut bnpcs_file, &bnpcs).unwrap();

        bnpcs
    } else {
        let bnpcs_file = BufReader::new(File::open(&args.bnpc_path).unwrap());
        serde_json::from_reader::<_, BNpcContainer>(bnpcs_file).unwrap()
    };

    // initialise ironworks
    let ironworks =
        Arc::new(Ironworks::new().with_resource(SqPack::new(Install::at(&args.game_path))));
    let excel = Excel::new(Arc::clone(&ironworks)).with_default_language(Language::English);

    // main object
    let mut affects = Affects::default();
    let mut name_map: BTreeMap<String, u16> = Default::default();

    let mut ctx = GeneratorContext {
        affects: &mut affects,
        excel: &excel,
        ironworks: &ironworks,
        name_map: &mut name_map,
        bnpcs: &bnpcs,
    };

    let overall = Instant::now();

    time("Items", || {
        crate::analysers::analyse_items(&mut ctx);
    });

    time("Emotes", || {
        crate::analysers::analyse_emotes(&mut ctx);
    });

    time("Battle NPCs", || {
        crate::analysers::analyse_bnpcs(&mut ctx);
    });

    time("Event NPCs", || {
        crate::analysers::analyse_enpcs(&mut ctx);
    });

    time("Actions", || {
        crate::analysers::analyse_actions(&mut ctx);
    });

    time("Minions", || {
        crate::analysers::analyse_minions(&mut ctx);
    });

    time("Mounts", || {
        crate::analysers::analyse_mounts(&mut ctx);
    });

    time("Ornaments", || {
        crate::analysers::analyse_ornaments(&mut ctx);
    });

    time("Maps", || {
        crate::analysers::analyse_maps(&mut ctx);
    });

    time("Equipment IMC", || {
        crate::analysers::imc::analyse_equipment_imcs(&mut ctx);
    });

    time("Weapon IMC", || {
        crate::analysers::imc::analyse_weapon_imcs(&mut ctx);
    });

    time("Monster IMC", || {
        crate::analysers::imc::analyse_monster_imcs(&mut ctx);
    });

    println!("=== {}ms overall ===", overall.elapsed().as_millis());

    time("Saving", || {
        let mut affects_file = BufWriter::new(File::create(&args.output).unwrap());
        if args.pretty {
            serde_json::to_writer_pretty(&mut affects_file, &affects).unwrap();
        } else {
            serde_json::to_writer(&mut affects_file, &affects).unwrap();
        }
    });

    println!("Done.");
}
