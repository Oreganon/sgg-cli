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

    let mut conn = Connection::new_dev(&include_str!("../cookie").replace("\n", "")).unwrap();
    &conn.send(&message);

    loop {
        let msg = &conn.read_msg().unwrap();
        println!("{}: {}", msg["nick"], msg["data"]);
    }
}
