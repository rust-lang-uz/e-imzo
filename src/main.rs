use e_imzo_rs::list_all_certificates;

fn main() {
    env_logger::init();
    match list_all_certificates() {
        Ok(pfx) => {
            let a: Vec<_> = pfx.iter().map(|c| (c, c.get_alias())).collect();
            println!("this is resut list_all_certificates; {:?}", a);
            pfx.iter().map(|c| (c, c.get_alias())).for_each(|(c, a)| {
                println!("CERT: {c:?}");
                println!("ALIAS: {a:?}");
                println!("-----");
            });
        }
        _ => {}
    }
}
