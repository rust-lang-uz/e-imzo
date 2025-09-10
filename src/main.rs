use e_imzo::{EIMZO, Result};

fn main() -> Result<()> {
    env_logger::init();
    let mut eimzo = EIMZO::new()?;

    match eimzo.list_all_certificates() {
        Ok(pfx) => {
            let a: Vec<_> = pfx.iter().map(|c| (c, c.get_alias())).collect();
            println!("this is resut list_all_certificates; {a:?}");
            pfx.iter().map(|c| (c, c.get_alias())).for_each(|(c, a)| {
                let validfrom: Vec<_> = a.get("validfrom").unwrap().split(" ").collect();
                let mut year_month_day: Vec<_> = validfrom[0].split(".").collect();
                year_month_day.reverse();

                println!("CERT: {c:?}");
                println!("ALIAS: {a:?}");
                println!("-----");
                println!("DATE: {:?}", year_month_day.join("."));
            });
        }
        Err(e) => println!("{e}"),
    }

    Ok(())
}
