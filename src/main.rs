use std::{ collections::HashSet, error::Error, fs, process::Command };

use crate::html_graph::html_graph::generate;

pub mod html_graph;

fn command_line(pdf_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    // Run pdftotext command
    let output = Command::new("pdftotext").args(&[pdf_path, "-", "-layout"]).output()?;

    // Convert output bytes to string
    let text = String::from_utf8_lossy(&output.stdout);
    print!(" {} PDF to text conversion successful", pdf_path);

    // Extract lines of text and return as Vec<&str>
    let lines: Vec<String> = text
        .split('\n')
        .map(|line| line.to_string())
        .collect();
    Ok(lines)
}

#[derive(Debug)]
#[derive(Clone)]
struct Record {
    id: i32,
    filename: String,
    date: String, // You can use other date/time types as needed
    description: String,
    value: f64, // Use a suitable type for monetary values
    org: String,
    full_date_string: String,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Number_Set {
    name: String,
    numbers: Vec<f64>,
}

struct Data {
    records: Vec<Record>,
    items: HashSet<String>,
    dates: HashSet<String>,
}

fn process_line(line: String, index: i32, filename: String) -> Record {
    // println!(
    //     "4678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890"
    // );
    // println!(
    //     "     6         7         8         9         0         1         2         3         4         5         6         7         8         9"
    // );
    // println!("{}.", &line[55..]);
    let date = &line[55..70].trim();
    let item = &line[70..100].trim();

    let deduct_string_full = (if line.len() > 138 { &line[119..139] } else { &line[119..] })
        .trim()
        .replace(',', "");
    // remove last . perhaps this needs to be in a separate function???
    let deduct_string =
        &deduct_string_full
            [..deduct_string_full.len() - (if deduct_string_full.ends_with('.') { 1 } else { 0 })];
    let addition_string_full = (if line.len() > 160 { &line[139..160] } else { &line[120..] })
        .trim()
        .replace(',', "");
    let addition_string =
        &addition_string_full
            [
                ..addition_string_full.len() -
                    (if addition_string_full.ends_with('.') { 1 } else { 0 })
            ];
    let deduct = match deduct_string.parse::<f64>() {
        Ok(num) => num,
        Err(_) => 0.0, // Default value on error
    };
    let addition = if deduct > 0.0 {
        0.0
    } else {
        match addition_string.parse::<f64>() {
            Ok(num) => num,
            Err(_) => 0.0, // Default value on error
        }
    };
    let value = if deduct > 0.0 { -deduct } else { addition };
    // println!("{}.", index);
    // println!("date->{}", date);
    // println!("item->{}", item);
    // println!("deduct->{}", deduct_string);
    // println!("add->{}", addition_string);
    // println!("value->{}", value);
    // println!("");
    // println!("");
    // println!("================================================================");
    if deduct_string.len() == 0 && addition_string.len() == 0 {
        panic!("parse error this line has no movement?!!");
    }
    if deduct > 0.0 && addition > 0.0 {
        panic!("parse error this line has two movement?!!");
    }
    Record {
        id: index,
        full_date_string: "".to_string(),
        filename: filename.clone(),
        date: date.to_string(),
        description: item.to_string(),
        value: value,
        org: line[55..].to_owned(),
    }
}

fn construct_full_date(mut rec: Record, statement_year: String, in_jan: bool) -> String {
    let months = [
        "not Zero",
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let month = rec.date[2..].trim();
    let index = months
        .iter()
        .position(|&r| r == month)
        .unwrap();
    let mut month_string = if index > 9 { String::from("") } else { String::from("0") };
    month_string.push_str(&index.to_string());
    if in_jan && index == 12 {
        let year: i32 = statement_year.parse().unwrap();
        let last_year = year - 1;
        rec.full_date_string.push_str(&last_year.to_string());
    } else {
        rec.full_date_string.push_str(&statement_year);
    }
    rec.full_date_string.push_str("-");
    rec.full_date_string.push_str(&month_string);
    rec.full_date_string.push_str("-");
    rec.full_date_string.push_str(&rec.date[0..2]);
    if rec.full_date_string.contains(" ") || rec.full_date_string.len() < 10 {
        panic!("{} -- rec date wrong {:?}", statement_year, rec);
    }
    // println!("{} from to  {} using {} year", rec.date, rec.full_date_string, statement_year);
    rec.full_date_string
}

fn process_lines(lines: Vec<String>, filename: String) -> Data {
    println!(" with {} lines", lines.len());
    let mut index = 1;
    let mut found_closing = false;
    let mut found_forward = false;
    let mut in_jan = false;
    let mut statement_year = String::new();

    let mut items: HashSet<String> = HashSet::new();
    let mut dates: HashSet<String> = HashSet::new();
    let mut records: Vec<Record> = Vec::with_capacity(lines.len()); // Pre-allocate space
    for line in lines {
        if line.contains("Statement date") {
            let start = &line.find("Statement date").unwrap() + "Statement date".len();
            let end = start + 16;
            let statement_date = if line.len() < end {
                line[25..].trim()
            } else {
                line[25..end].trim()
            };
            in_jan = line.contains("January");
            statement_year = String::from("20");
            statement_year.push_str(&statement_date[statement_date.len() - 2..]);
            // println!("{} ", line);
            // println!("statement date {} and year {} ", statement_date, statement_year);
        }
        let short = line.len() < 60;
        if line.contains("closing balance") {
            found_closing = true;
        }
        if found_forward && !found_closing && !short {
            // && index == 32 {
            let rec = process_line(line.clone(), index, filename.clone());
            // maybe reimplement for speed?
            // let empty_year = statement_year.is_empty();
            // if !empty_year {
            //     construct_full_date(rec.clone(), statement_year.to_string());
            //     let date = &rec.full_date_string.clone();
            //     dates.insert(date.to_string());
            // }
            let desc = rec.clone().description;
            records.push(rec);
            items.insert(desc);
        }

        if line.contains("BROUGHT FORWARD") {
            found_forward = true;
        }
        index = index + 1;
    }
    // for i in 0..index_Year as usize {
    for i in 0..records.len() {
        records[i].full_date_string = construct_full_date(
            records[i].clone(),
            statement_year.to_string(),
            in_jan
        );
        dates.insert(records[i].full_date_string.to_string());
    }
    return Data {
        records: records,
        items: items,
        dates: dates,
    };
}

fn main() {
    // @TODO find statement date and work out the year, then parse dates correctly into rust dates.

    //    let pdf_path = "../Documents/coop/5bee1b4c-a758-4ed7-8a35-96858f398e06.pdf";
    let paths = fs::read_dir("../Documents/coop/").unwrap();
    let mut records: Vec<Record> = Vec::new();
    let mut items: HashSet<String> = HashSet::new();
    let mut dates: HashSet<String> = HashSet::new();
    for path in paths {
        let filename = path.unwrap();
        let mut path_name = "../Documents/coop/".to_owned();
        path_name.push_str(&filename.file_name().into_string().unwrap());
        if path_name.contains("pdf") {
            match command_line(&path_name) {
                //match command_line(pdf_path) {
                Ok(lines) => {
                    let mut data = process_lines(lines, path_name);
                    records.append(&mut data.records);
                    items.extend(data.items);
                    dates.extend(data.dates);
                }
                Err(err) => {
                    eprintln!("Error: pdftotext command failed {}", err);
                }
            }
        }
    }

    let mut date_list: Vec<String> = dates.into_iter().collect();
    date_list.sort_by(|a, b| a.partial_cmp(&b).unwrap());

    let mut data = vec![0.0; date_list.len()];

    let mut item_list: Vec<String> = items.into_iter().collect();
    item_list.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut filtered_records: Vec<&Record> = records
        .iter()
        .filter(|r| r.description.contains("TFR"))
        // .filter(|r| r.description.contains("GOCARDLESS"))
        .collect();

    for record in filtered_records {
        let pos = date_list
            .iter()
            .position(|s| s == &record.full_date_string)
            .unwrap();
        data[pos] = record.value;
    }

    println!("We now have {} records", records.len());
    let mut numbers_sets: Vec<Number_Set> = Vec::new();
    let number_set: Number_Set = Number_Set {
        name: "Transfers".to_string(),
        numbers: data,
    };
    numbers_sets.push(number_set.clone());
    println!("data {:?}", numbers_sets);
    println!("labels {:?}", date_list);
    generate("WHAT.html", numbers_sets, date_list);
}
