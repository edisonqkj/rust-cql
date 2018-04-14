extern crate cql;

fn run() -> cql::Result<()> {
    let mut client = cql::Client::new("localhost:9042")?;

    eprintln!("ready");

    let res = client.options();
    println!("options: {:?}", res);

    let res = client.query(
        "create keyspace rust with replication = \
         {'class': 'SimpleStrategy', 'replication_factor':1}",
        cql::Consistency::One,
    )?;
    println!("create: {:?}", res);

    let res = client.query(
        "create table rust.test (v1 text primary key, v2 float, v3 boolean)",
        cql::Consistency::One,
    )?;
    println!("create table: {:?}", res);

    let res = client.query(
        "insert into rust.test (v1, v2, v3) values ('asdf', 1.2345, false)",
        cql::Consistency::One,
    )?;
    println!("insert: {:?}", res);

    let res = client.query("select * from rust.test", cql::Consistency::One)?;
    println!("select: {:?}", res);

    Ok(())
}

fn main() {
    match run() {
        Ok(()) => {}
        Err(e) => {
            println!("Error while running CQL: {:?}", e);
        }
    }
}
