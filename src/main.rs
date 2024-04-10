use std::{ error::Error, fs, process::Command };

fn command_line(pdf_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    // Run pdftotext command
    let output = Command::new("pdftotext").args(&[pdf_path, "-", "-layout"]).output()?;

    // Convert output bytes to string
    let text = String::from_utf8_lossy(&output.stdout);
    println!(" {} PDF to text conversion successful", pdf_path);

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
    addition: f64, // Use a suitable type for monetary values
    deduct: f64,
    org: String,
}

fn process_lines(lines: Vec<String>, filename: String) -> Vec<Record> {
    println!(" with {} lines", lines.len());
    let mut index = 1;
    let mut found_closing = false;
    let mut found_forward = false;

    let mut records: Vec<Record> = Vec::with_capacity(lines.len()); // Pre-allocate space
    for line in lines {
        if line.contains("BROUGHT FORWARD") {
            found_forward = true;
        }
        let short = line.len() < 60;
        if found_forward && !found_closing && !short {
            println!(
                "4678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890"
            );
            println!(
                "     6         7         8         9         0         1         2         3         4         5         6         7         8         9"
            );
            println!("{}.", &line[55..]);
            let date = &line[55..70].trim();
            let item = &line[70..100].trim();
            let deduct_string = (if line.len() > 131 { &line[120..130] } else { &line[120..] })
                .trim()
                .replace(',', "");
            let addition_string = (if line.len() > 160 { &line[140..160] } else { "" })
                .trim()
                .replace(',', "");
            let deduct = match deduct_string.parse::<f64>() {
                Ok(num) => num,
                Err(_) => 0.0, // Default value on error
            };
            let addition = match addition_string.parse::<f64>() {
                Ok(num) => num,
                Err(_) => 0.0, // Default value on error
            };
            println!("{}.", index);
            println!("date->{}", date);
            println!("item->{}", item);
            println!("deduct->{}", deduct_string);
            println!("add->{}", addition_string);
            println!("");
            println!("");
            println!("================================================================");
            records.push(Record {
                id: index,
                filename: filename.clone(),
                date: date.to_string(),
                description: item.to_string(),
                addition: addition,
                deduct: deduct,
                org: line[55..].to_owned(),
            });
        }
        if line.contains("closing balance") {
            found_closing = true;
        }
        index = index + 1;
    }
    return records;
}

fn main() {
    let pdf_path = "../Documents/coop/5bee1b4c-a758-4ed7-8a35-96858f398e06.pdf";
    let paths = fs::read_dir("../Documents/coop/").unwrap();
    let mut records: Vec<Record> = Vec::new();
    for path in paths {
        let filename = path.unwrap();
        let mut path_name = "../Documents/coop/".to_owned();
        path_name.push_str(&filename.file_name().into_string().unwrap());
        if path_name.contains("pdf") {
            match command_line(&path_name) {
                //match command_line(pdf_path) {
                Ok(lines) => {
                    let mut recs = process_lines(lines, path_name);
                    records.append(&mut recs);
                }
                Err(err) => {
                    eprintln!("Error: pdftotext command failed {}", err);
                }
            }
        }
    }
    println!("We now have {:#?} ", records);
    println!("We now have {} records", records.len());
    let brams: Vec<&Record> = records
        .iter()
        .filter(|r| r.description == "BRAMHILL P&M")
        .collect();
    for bram in brams {
        println!("Just brum {:?}", bram);
    }
    let mut filtered_records: Vec<&Record> = records
        .iter()
        .filter(|r| r.description.contains("TFR"))
        .collect();

    // filtered_records.sort_by(|a, b| a.date.partial_cmp(&b.date).unwrap());
    filtered_records.sort_by(|a, b| a.id.partial_cmp(&b.id).unwrap());

    let trans = filtered_records.clone();
    // let trans: Vec<&Record> = filtered_records.sort_by(|a, b| a.date.partial_cmp(&b.date).unwrap());

    // .sort_by(|a, b| a.id - b.id);
    for tr in trans {
        println!("{:?}", tr);
    }

    let mut frec: Vec<&Record> = records
        .iter()
        .filter(|r| r.filename.contains("f67a1c0-21f5-4a82-b46b-c1079e170d08"))
        .collect();

    frec.sort_by(|a, b| a.id.partial_cmp(&b.id).unwrap());

    for tr in frec {
        println!("{}", tr.org);
        println!("{}", tr.addition);
    }
}
