Extremely basic CSV to vCard converter.

CSV has 4 required headers: First Name, Last Name, Phone, Email
If any of those are missing an exception will be thrown.


How to use:
-----

- Download [the binary](https://github.com/ptrckSTL/csv_to_vcard/blob/master/csv_to_vcard.exe)
- Put contacts.csv in the same directory as the binary
- run the binary

or

- install [Cargo]("https://doc.rust-lang.org/cargo/getting-started/installation.html")
- [download](https://github.com/ptrckSTL/csv_to_vcard/archive/refs/heads/master.zip) and unzip this project
- Put contacts.csv in the same directory as the project
- open terminal in directory
- cargo run

TODO
--

- helpful error handling
- make fields optional
- support more fields
- support for path to contacts