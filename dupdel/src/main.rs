use std::collections::HashSet;
use clap::{App, Arg};

fn main() {

    let matches = App::new("dupdel")
        .version("1.0")
        .author("Maebli")
        .about("Finds duplicate files in current directory recursively")
        .arg(Arg::with_name("folder")
            .short('f')
            .long("folder")
            .help("Specify the folder in which the duplicates should be searched")
            .takes_value(true))
        .get_matches();

    let folder = matches.value_of("folder").unwrap_or(".");

    let hash_set = HashSet::new();

    find_duplicates(hash_set, folder);

}

fn find_duplicates(mut hash_set: HashSet<Vec<u8>>, folder: &str) -> HashSet<Vec<u8>>{

    if let Some(folder) = std::fs::read_dir(folder).ok() {
        for entry in folder {
            if let Ok(entry) = entry {
                if entry.path().exists() {
                    if entry.path().is_dir() {
                        hash_set = find_duplicates(hash_set, entry.path().to_str().unwrap());
                    } else {
                        let file = std::fs::read(entry.path()).unwrap();
                        if hash_set.contains(&file) {
                            println!("🪡 Found duplicate: {}", entry.path().to_str().unwrap());
                        } else {
                            hash_set.insert(file);
                        }
                    }
                }
            }
        }
    }

    hash_set
}

