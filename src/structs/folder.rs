pub struct Folder<'a> {
    pub path: &'a str,
    pub db_entry: &'a str,
    pub set_entries: [&'a str; 2],
}
