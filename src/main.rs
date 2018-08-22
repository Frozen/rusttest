
#[macro_use]
extern crate structopt;
extern crate postgres;
extern crate chrono;


use chrono::{Duration};
use chrono::naive::NaiveDateTime;
use std::collections::HashMap;

use postgres::{Connection, TlsMode};
//use std::env;

use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag.
    /// year parse
    #[structopt(short = "y", long = "year")]
    year: i32,

    #[structopt(short = "c", long = "conn")]
    conn_url: String

//    /// action to execute
//    #[structopt(short = "a", long = "action")]
//    action: String
}


fn main() {

    let opt = Opt::from_args();
    println!("{:?}", opt);


    let query = format!("select price, date_trunc('second', moment) as moment from rts where moment >= '{}-01-01' and moment < '{}-01-01' order by moment", opt.year, opt.year + 1);
//    let query = format!("select price, moment from rts where moment >= '{}-01-01' and moment < '{}-01-01'", opt.year, opt.year + 1);


    let conn = Connection::connect(&*opt.conn_url, TlsMode::None).unwrap();

    let mut moment_price: HashMap<NaiveDateTime, Vec<f64>> = HashMap::new();

    for row in &conn.query(&query, &[]).unwrap() {
        moment_price.entry(row.get(1)).or_insert_with(Vec::new ).push(row.get(0));
    }




    for (key, value) in &moment_price {
        let r = conn.execute("insert into rts_grouped_minute values($1, $2, $3, $4, $5)",
                     &[
                         key,
                         &min(value),
                         &max(value),
                         value.first().unwrap(),
                         value.last().unwrap(),
                         moment_price.get(&(key.clone() - Duration::seconds(1))).map(|e| e.last().unwrap()).unwrap_or(&0.0)]);

        r.unwrap();
    }


}

fn max(my_vec: &[f64]) -> f64 {

    let mut m = 0.0;

    for x in my_vec {
        if *x > m {
            m = *x;
        }
    }

    return m;
}


fn min(my_vec: &[f64]) -> f64 {

    let mut m: f64 = 1_000_000_000.0;

    for x in my_vec {
        if *x < m {
            m = *x;
        }
    }

    return m;
}

//fn max() -> f64 {
//    my_vec.into_iter().fold(None, |min, x| match min {
//        None => Some(x),
//        Some(y) => Some(if x.my_f32 < y.my_f32 { x } else { y }),
//    })
//}



