use spin_sdk::http::{IntoResponse, Request};
use spin_sdk::http_component;

mod spin_sqlx;

use sqlx::Row;

struct Person {
    name: String,
}

struct Pet {
    age: u32,
    name: String,
    is_finicky: bool,
    real_thingy: f32,
}

// Not traited
// struct Person2 {
//     name: String,
// }

impl<'r> sqlx::FromRow<'r, spin_sqlx::SpinSqliteRow> for Person {
    fn from_row(row: &'r spin_sqlx::SpinSqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            name: row.try_get("name")?,
        })
    }
}

impl<'r> sqlx::FromRow<'r, spin_sqlx::SpinSqliteRow> for Pet {
    fn from_row(row: &'r spin_sqlx::SpinSqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            age: row.try_get("age")?,
            name: row.try_get("name")?,
            is_finicky: row.try_get("is_finicky")?,
            real_thingy: row.try_get("real_thingy")?,
        })
    }
}

impl std::fmt::Display for Pet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fdesc = if self.is_finicky { "is" } else { "is not" };
        f.write_fmt(format_args!("{}, aged {}, {} finicky and uh yeah real {}", self.name, self.age, fdesc, self.real_thingy))
    }
}

/// A simple Spin HTTP component.
#[http_component]
async fn handle_sqlxtest(_req: Request) -> anyhow::Result<impl IntoResponse> {

    // let conn = spin_sdk::sqlite::Connection::open_default()?;
    // let rs = conn.execute("INSERT INTO test(name) VALUES ('spork')", &[])?;
    // let rs = conn.execute("SELECT changes()", &[])?;
    // for c in &rs.columns {
    //     println!("COL={c}");
    // }
    // for r in &rs.rows {
    //     println!("ROWLEN={}", r.values.len());
    // }

    let sqlx_conn = spin_sqlx::SqlxConnection::open_default()?;

    // let _qr = sqlx::query("INSERT INTO test(name) VALUES ('honk')")
    //     .execute(&sqlx_conn)
    //     .await?;
    let _qr = sqlx::query("INSERT INTO pets(age, name, is_finicky, real_thingy) VALUES (?, ?, ?, ?)")
        .bind(18)
        .bind("Slats")
        .bind(true)
        .bind(23.456)
        .execute(&sqlx_conn)
        .await?;

    // We can't do anything with QR - SQLite does not return rows affected or
    // any such jollity

    // let rs = sqlx::query_as::<_, Person>("SELECT name FROM test")
    // // let rs = sqlx::query_as!(Person2, "SELECT name FROM test")  // cargo sqlx prepare complains "no database driver found matching URL scheme"
    //     .fetch(&sqlx_conn);
    // use futures::stream::StreamExt;
    // //let strs = rs.map(|r| r.unwrap().name);
    // let resp = rs.fold("".to_owned(), |acc, s| async move { format!("{acc}, {}", s.unwrap().name) }).await;

    // let rs = sqlx::query_as::<_, Person>("SELECT name FROM test WHERE name = ?")
    //     .bind("honk")
    //     .fetch(&sqlx_conn);
    // use futures::stream::StreamExt;
    // // let strs = rs.map(|r| r.unwrap().try_get::<String, _>(0));
    // // let resp = strs.fold("".to_owned(), |acc, s| async move { format!("{acc}, {}", s.unwrap()) }).await;
    // let resp = rs.fold("".to_owned(), |acc, s| async move { format!("{acc}, {}", s.unwrap().name) }).await;

    // let rs = sqlx::query("SELECT name FROM test")
    //     // .fetch_all(sqlx_conn)
    //     // .await?;
    //     .fetch(&sqlx_conn);

    // // with .fetch_all().await()?
    // // println!("ROW COUNT: {}", rs.len());
    // // let strs = rs.iter().map(|r| r.try_get::<String, _>(0));
    // // let resp = strs.collect::<Result<Vec<_>, _>>()?.join(",");

    // let p = sqlx::query_as::<_, Person>("SELECT name FROM test WHERE name = ?")
    //     .bind("honk")
    //     .fetch_one(&sqlx_conn)
    //     .await?;
    // let resp = format!("'{}' is the name, {}ing's the game", p.name, p.name);

    let ps = sqlx::query_as::<_, Pet>("SELECT * FROM pets")
        .fetch(&sqlx_conn);
    use futures::stream::StreamExt;
    let resp = ps.fold("".to_owned(), |acc, pet| async move { format!("{acc}, {}", pet.unwrap()) }).await;
    // // with fetch()
    // use futures::stream::StreamExt;
    // let strs = rs.map(|r| r.unwrap().try_get::<String, _>(0));
    // let resp = strs.fold("".to_owned(), |acc, s| async move { format!("{acc}, {}", s.unwrap()) }).await;

    Ok(http::Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(format!("{resp}\n"))?)
}
