#[macro_use(concat_string)]
extern crate concat_string;

use std::fs;
use std::fs::File;
use std::io::Write;

use csv::{Reader, StringRecord};

use columns::column_data::Columns;

mod columns;

fn main() {
    let mut result = "".to_string();
    let csv = parse_csv();
    let bytes = csv.as_bytes();
    let mut sheets = Reader::from_reader(bytes);
    let headers = sheets.headers().unwrap();
    let columns = get_required_columns(&headers);
    for record in sheets.records() { //excludes headers
        let record = record.unwrap();
        let f_n = &record[usize::from(columns.first_name)];
        let l_n = &record[usize::from(columns.last_name)];
        let email = &record[usize::from(columns.email)];
        let phone = &record[usize::from(columns.phone)];
        result.push_str(vcard_from_strings(f_n, l_n, phone, email).as_str());
    }
    let mut vcard = File::create("contacts.vcf").unwrap();
    match vcard.write_all(result.as_bytes()) {
        Ok(_) => {
            println!("Successfully wrote contacts.vcf");
        }
        Err(e) => {
            println!("Failed writing to disk\nReason: {}", e);
        }
    }
}

fn parse_csv() -> String {
    match fs::read_to_string("contacts.csv") {
        Ok(str) => { str }
        Err(_) => { panic!("Could not parse csv") }
    }
}

fn get_required_columns(records: &StringRecord) -> Columns {
    Columns::build_columns(records)
}


fn vcard_from_strings(first_name: &str, last_name: &str, phone: &str, email: &str) -> String {
    let vcard_start = "BEGIN:VCARD\nVERSION:3.0\n";
    let vcard_name = concat_string!("N:*",last_name,";",first_name,";;;\n");
    let vcard_full = concat_string!("FN:*",last_name, " ", first_name,"\n");
    let vcard_cell = concat_string!("TEL;TYPE=CELL:",phone,"\n");
    let vcard_email = concat_string!("EMAIL:",email,"\n");
    let vcard_end = "END:VCARD\n";
    concat_string!(vcard_start,vcard_name,vcard_full,vcard_cell,vcard_email,vcard_end)
}
