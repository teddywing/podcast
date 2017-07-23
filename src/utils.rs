use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::num::ParseIntError;


pub fn already_downloaded(dir: &str) -> BTreeSet<String> {
    let mut result = BTreeSet::new();

    let mut path = get_podcast_dir();
    path.push(dir);

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                match entry.file_name().into_string() {
                    Ok(val) => {
                        let name = String::from(val);
                        let index = name.find('.').unwrap();
                        result.insert(String::from(&name[0..index]));
                    }
                    Err(_) => {
                        println!(
                            "OsString: {:?} couldn't be converted to String",
                            entry.file_name()
                        );
                    }
                }
            }
        }
    }
    result
}

pub fn get_podcast_dir() -> PathBuf {
    match env::var_os("PODCAST") {
        Some(val) => PathBuf::from(val),
        None => {
            let mut path = env::home_dir().unwrap();
            path.push("Podcasts");
            path
        }
    }
}

pub fn get_sub_file() -> PathBuf {
    match env::var_os("PODCAST") {
        Some(val) => {
            let mut path = PathBuf::from(val);
            path.push(".subscriptions");
            path
        }
        None => {
            let mut path = env::home_dir().unwrap();
            path.push("Podcasts");
            path.push(".subscriptions");
            path
        }
    }
}

pub fn parse_download_episodes(e_search: &str) -> Result<Vec<usize>, ParseIntError> {
    let input = String::from(e_search);
    let mut ranges = Vec::<(usize, usize)>::new();
    let mut elements = Vec::<usize>::new();
    let comma_separated: Vec<&str> = input.split(',').collect();
    for elem in comma_separated {
        let temp = String::from(elem);
        if temp.contains("-") {
            let range: Vec<usize> = elem.split('-')
                .map(|i| i.parse::<usize>().unwrap())
                .collect();
            ranges.push((range[0], range[1]));
        } else {
            elements.push(elem.parse::<usize>()?);
        }
    }

    for range in ranges {
        // Add 1 to upper range to include given episode in the download
        for num in range.0..range.1 + 1 {
            elements.push(num);
        }
    }
    elements.dedup();
    Ok(elements)
}
