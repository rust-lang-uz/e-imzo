// use chrono::{DateTime, Utc};
use e_imzo_rs::list_all_certificates;

fn main() {
    env_logger::init();
    match list_all_certificates() {
        Ok(pfx) => {
            let a: Vec<_> = pfx.iter().map(|c| (c, c.get_alias())).collect();
            println!("this is resut list_all_certificates; {a:?}");
            pfx.iter().map(|c| (c, c.get_alias())).for_each(|(c, a)| {
                // let validfrom: DateTime<Utc> = a.get("validfrom").unwrap().parse().unwrap();
                println!("CERT: {c:?}");
                println!("ALIAS: {a:?}");
                println!("-----");
                // println!("DATE: {validfrom:?}");
            });
        }
        Err(e) => println!("{e}"),
    }
    // let ws_url = Url::parse("wss://127.0.0.1:64443/service/cryptapi");
}
