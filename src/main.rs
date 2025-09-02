use chrono::{DateTime, Utc};
// use chrono::{DateTime, Utc};
use e_imzo::list_all_certificates;

fn main() {
    // env_logger::init();
    // match list_all_certificates() {
    //     Ok(pfx) => {
    //         let a: Vec<_> = pfx.iter().map(|c| (c, c.get_alias())).collect();
    //         println!("this is resut list_all_certificates; {a:?}");
    //         pfx.iter().map(|c| (c, c.get_alias())).for_each(|(c, a)| {
    //             let validfrom: Vec<_> = a.get("validfrom").unwrap().split(" ").filter_map(|date| {
    //                 let year_month_day: Vec<_> = date.split(".").collect();
    //                 Some(year_month_day)

    //             }).collect();
    //             // let valid_date: Vec<_> = validfrom[0].split(".").collect();

    //             println!("CERT: {c:?}");
    //             println!("ALIAS: {a:?}");
    //             println!("-----");
    //             println!("DATE: {:?}", validfrom);
                
    //         });
    //     }
    //     Err(e) => println!("{e}"),
    // }
    // let ws_url = Url::parse("wss://127.0.0.1:64443/service/cryptapi");
}
