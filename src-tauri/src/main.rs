#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use pdf_extract::*;
use regex;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Swimmer {
    name: String,
    year: String,
    club: String,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Lane {
    lane: String,
    swimmer: Swimmer,
    time: String,
    byte_offset: usize,
}
#[derive(Debug, Eq, PartialEq, Clone)]
struct Run {
    run: String,
    time: String,
    lane_list: Vec<Lane>,
    byte_offset: usize,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Competition {
    competition: String,
    run_list: Vec<Run>,
    byte_offset: usize,
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![process_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn process_file(filepath: String, clubname: String) -> String {
    // println!("{}", filepath);

    // Read the file
    let content = load_file(&filepath);

    // Set Clubname
    let club_name = clubname;

    // Remove filename from filepath
    let file_path = filepath.replace(&filepath.split("\\").last().unwrap(), "");

    // Get Filename without extension from filepath
    let filename = filepath.split(".").next().unwrap();
    let filename = filename.split("\\").last().unwrap();

    // Set name for output file
    let output_name = format!("{}\\{}.csv", file_path, filename);

    // Process content
    let competitions = process_content(&content, &club_name);

    // Convert content to csv file
    convert_to_csv(competitions, &output_name);

    "Converted successfully".into()
}

/// Loads the file from the given path into a string
/// # Argumennt is the filepath as &str
/// # Output is the content of the file as String
fn load_file(filepath: &str) -> String {
    //File handling
    let content = match extract_text(&filepath) {
        Ok(data) => {
            // println!("{}", "Successfully loaded file.");
            data
        }
        Err(_) => {
            // println!("{}", "Problem opening the file.\nProgramm will exit.");
            return String::new();
        }
    };
    content
}

/// Takes a Vector from Competition and saves a formated .csv file in the root folder
/// # Arguments is a Vec<Comp>
/// # Output wk.csv
fn convert_to_csv(wk: Vec<Competition>, output_name: &str) {
    let mut csv_string = String::new();
    csv_string
      .push_str("WK;Uhrzeit;Lauf;Bahn;Name;Jahrgang;Verein;Zeit;Split 1;Split 2;Split 3;Split 4;Split 5;Split 6;Split 7;Split 8;\n");
    for w in wk {
        for l in w.run_list {
            for b in l.lane_list {
                csv_string.push_str(&w.competition);
                csv_string.push_str(";");
                csv_string.push_str(&l.time);
                csv_string.push_str(";");
                csv_string.push_str(&l.run);
                csv_string.push_str(";");
                csv_string.push_str(&b.lane);
                csv_string.push_str(";");
                csv_string.push_str(&b.swimmer.name);
                csv_string.push_str(";");
                csv_string.push_str(&b.swimmer.year);
                csv_string.push_str(";");
                csv_string.push_str(&b.swimmer.club);
                csv_string.push_str(";");
                csv_string.push_str(&b.time);
                csv_string.push_str(";;;;;;;;;\n");
            }
        }
    }
    std::fs::write(output_name, csv_string).unwrap();
}

fn process_content(content: &str, club_name: &str) -> Vec<Competition> {
    //Find all Wettkampf and there positions in the text
    let re_comp =
        regex::Regex::new(r"(Wettkampf\s\d+)\s-\s(\d*?\S*\d+\s*m\s+\S+)\s(\S.+)").unwrap();
    let mut comp_list: Vec<Competition> = Vec::new();
    re_comp.captures_iter(&content).for_each(|cap_comp| {
        let comp = Competition {
            competition: cap_comp[2].to_string(),
            run_list: Vec::new(),
            byte_offset: cap_comp.get(0).unwrap().start(),
        };
        comp_list.push(comp);
    });

    //Find all Lauf and there positions in the text
    let mut run_list: Vec<Run> = Vec::new();
    let re_run = regex::Regex::new(r"(Lauf\s+)(\d+)/(\d+)\s\(ca.\s(\d+:\d+)\sUhr\)").unwrap();
    re_run.captures_iter(&content).for_each(|cap_run| {
        let run = Run {
            run: cap_run[2].to_string(),
            time: cap_run[4].to_string(),
            lane_list: Vec::new(),
            byte_offset: cap_run.get(0).unwrap().start(),
        };

        run_list.push(run);
    });

    //Swimmer HashMap
    let mut swimmer_list: HashMap<String, Swimmer> = HashMap::new();

    //Find all Bahn and there positions in the text
    let mut lane_list: Vec<Lane> = Vec::new();
    let re_lane = regex::Regex::new(
    r"Bahn[ \t]+(\d+)[ \t]+([^\n]+)[ \t][ \t]+(\d+(?:/AK\s\d+)?|)[ \t]+([^\n]+)[ \t]+(\d+:\d+,\d+)",
)
.unwrap();
    re_lane.captures_iter(&content).for_each(|cap_lane| {
        let new_swimmer = Swimmer {
            name: cap_lane[2].trim_end().to_string(),
            year: cap_lane[3].to_string(),
            club: cap_lane[4].trim_end().to_string(),
        };

        let lane = Lane {
            lane: cap_lane[1].to_string(),
            swimmer: new_swimmer.clone(),
            time: cap_lane[5].to_string(),
            byte_offset: cap_lane.get(0).unwrap().start(),
        };

        if lane.swimmer.club == club_name.to_string() {
            swimmer_list.insert(cap_lane[2].trim_end().to_string(), new_swimmer);
            lane_list.push(lane);
        } else if club_name == "" {
            swimmer_list.insert(cap_lane[2].trim_end().to_string(), new_swimmer);
            lane_list.push(lane);
        }
    });

    //Add Bahn to the appropriate Lauf
    run_list.iter_mut().rev().for_each(|run| {
        run.lane_list.extend(
            lane_list
                .iter()
                .cloned()
                .filter(|lane| lane.byte_offset > run.byte_offset),
        );
        lane_list.retain(|lane| lane.byte_offset < run.byte_offset);
    });

    //Remove all empty lane_lists
    run_list.retain(|run| !run.lane_list.is_empty());

    //Add Run to the appropriate Competition
    comp_list.iter_mut().rev().for_each(|comp| {
        comp.run_list.extend(
            run_list
                .iter()
                .cloned()
                .filter(|run| run.byte_offset > comp.byte_offset),
        );
        run_list.retain(|run| run.byte_offset < comp.byte_offset);
    });

    //Remove all empty Wettkampf
    comp_list.retain(|comp| !comp.run_list.is_empty());

    // println!("Swimmers: {}", swimmer_list.len());
    // println!("{:#?}", swimmer_list);
    // println!("Runs: {}", run_list.len());
    // println!("Comps: {}", comp_list.len());
    // println!("{:#?}", comp_list);

    comp_list
}
