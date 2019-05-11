use std::env;
use std::process::exit;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::error::Error;

use simple_error::SimpleError;

// Parsing these on start up feels kinda bad and unnecessary
const FORT_LEADER_FILE_CONTENTS: &'static str = include_str!("../gamedata/fort_leader_types_by_nation.tsv");
const FORT_TROOP_FILE_CONTENTS: &'static str = include_str!("../gamedata/fort_troop_types_by_nation.tsv");

fn main() -> Result<(), Box<Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("CURRENT USAGE: just 3 args - base nation, then commanders, then troops");
        exit(-1);
    }
    let base_nation_id = args[1].parse::<usize>()?;
    let commander_nation_id = args[2].parse::<usize>()?;
    let troops_nation_id = args[3].parse::<usize>()?;


    let fort_commanders =
        load_monsters_by_nation_from_file(FORT_LEADER_FILE_CONTENTS)?;


    let fort_commanders_text =
        entries_by_line_with_prefix(
        "#addreccom ",
        commander_nation_id,
        &fort_commanders,
    )?;

    let fort_units =
        load_monsters_by_nation_from_file(FORT_TROOP_FILE_CONTENTS)?;

    let fort_unit_text =
        entries_by_line_with_prefix(
        "#addrecunit ",
        troops_nation_id,
        &fort_units,
    )?;

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

fn load_monsters_by_nation_from_file(file_contents: &'static str) -> Result<HashMap<usize, Vec<usize>>, Box<Error>> {
    let mut hash_map: HashMap<usize, Vec<usize>> = HashMap::new();

    let mut line_reader = file_contents.lines();
    let _ = line_reader.next().unwrap(); // skip heading
    for line in line_reader {
        let mut fields = line.split('\t');
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
    Ok(hash_map)
}
