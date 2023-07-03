pub mod column_data {
    use std::fmt;
    use std::fmt::Formatter;

    use csv::StringRecord;

    pub struct Columns {
        pub first_name: u8,
        pub last_name: u8,
        pub phone: u8,
        pub email: u8,
    }

    impl Columns {
        pub fn build_columns(record: &StringRecord) -> Columns {
            let mut first_name = None;
            let mut last_name = None;
            let mut phone = None;
            let mut email = None;

            for column in record {
                if let Some(idx) = record.iter().position(|f| f == column) {
                    match column {
                        "First Name" => first_name = Some(idx as u8),
                        "Last Name" => last_name = Some(idx as u8),
                        "Phone" => phone = Some(idx as u8),
                        "Email" => email = Some(idx as u8),
                        _ => {}
                    }
                }
            }

            Columns {
                first_name: first_name.unwrap(),
                last_name: last_name.unwrap(),
                phone: phone.unwrap(),
                email: email.unwrap(),
            }
        }
    }

    impl fmt::Display for Columns {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "first name: {}\nlast name: {}\nphone: {}\nemail: {}", self.first_name, self.last_name, self.phone, self.email)
        }
    }
}
