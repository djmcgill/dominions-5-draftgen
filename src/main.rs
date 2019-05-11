use std::env;
use std::path::Path;
use std::process::exit;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::error::Error;

use simple_error::*;

const FORT_LEADER_FILE: &'static str = "gamedata/fort_leader_types_by_nation.tsv";
const FORT_TROOP_FILE: &'static str = "gamedata/fort_troop_types_by_nation.tsv";

fn main() -> Result<(), Box<Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("CURRENT USAGE: just 3 args - base nation, then commanders, then troops");
        exit(-1);
    }
    let base_nation_id = try_with!(
        args[1].parse::<usize>(),
        "Could not parse base_nation_id"
    );
    let commander_nation_id = try_with!(
        args[2].parse::<usize>().unwrap(),
        "Could not parse commander_nation_id"
    );
    let troops_nation_id = try_with!(
        args[3].parse::<usize>().unwrap(),
        "Could not parse troops_nation_id"
    );

    let fort_commanders = try_with!(
        load_monsters_by_nation_from_file(FORT_LEADER_FILE)
        "Could not load file {}", FORT_LEADER_FILE
    );

    let fort_commanders_text = try_with!(
        entries_by_line_with_prefix(
            "#addreccom ",
            commander_nation_id,
            &fort_commanders,
        ),
        "Could not generate text from fort_commanders data"
    );

    let fort_units = try_with!(
        load_monsters_by_nation_from_file(FORT_TROOP_FILE),
        "Could not load file {}", FORT_TROOP_FILE
    );

    let fort_unit_text = try_with!(
        entries_by_line_with_prefix(
            "#addrecunit ",
            troops_nation_id,
            &fort_units,
        ),
        "Could not generate text from fort_troops data"
    );

    println!("
#modname \"dumb draft mod\"
#selectnation {base_nation_id}
#clearrec

{fort_commander_text}

{fort_unit_text}

#end
#end", base_nation_id=base_nation_id,
       fort_commander_text=fort_commanders_text,
        fort_unit_text=fort_unit_text,
    );
    Ok(())
}

fn entries_by_line_with_prefix(prefix: &str, key: usize, hash_map: &HashMap<usize, Vec<usize>>) -> Result<String, Box<Error>> {
    let mut text = String::new();
    hash_map.get(&key).map(|values| {
        for &value in values {
            text.push_str(&format!("{}{}\n", prefix, value));
        }
    }).ok_or_else(|| SimpleError::new(format!("Could not find key '{}' in map", key)))?;
    Ok(text)
}

fn load_monsters_by_nation_from_file<P: AsRef<Path>>(path: P) -> Option<HashMap<usize, Vec<usize>>> {
    let mut hash_map: HashMap<usize, Vec<usize>> = HashMap::new();
    let file = File::open(path).unwrap_or_else(|_| {
        panic!("COULD NOT OPEN FILE");
    });

    let mut line_reader = BufReader::new(file).lines();
    let _ = line_reader.next().unwrap(); // skip heading
    for line in line_reader {
        let actual_line = line.unwrap();
        let mut fields = actual_line.split('\t');
        let monster_number = fields.next().unwrap().parse::<usize>().unwrap();
        let nation_id = fields.next().unwrap().parse::<usize>().unwrap();
        let entry = hash_map.entry(
            nation_id,
        );
        match entry {
            Entry::Occupied(mut occ_entry) => {
                occ_entry.get_mut().push(monster_number);
            }
            Entry::Vacant(vac_entry) => {
                vac_entry.insert(vec![monster_number]);
            }
        };
    }
    Some(hash_map)
}
