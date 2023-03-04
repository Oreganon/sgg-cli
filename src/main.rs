use std::env;
use wsgg::Connection;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() >= 2);
    let message = args
        .iter()
        .skip(1)
        .map(|x| x.as_str())
        .collect::<Vec<&str>>()
        .join(" ");
    let conn = Connection::new(&include_str!("../cookie").replace("\n", "")).unwrap();
    conn.send(&message);
}
