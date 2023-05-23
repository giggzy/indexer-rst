// Module: db
use crate::xml::{Page, Revision};
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct PageRow {
    title: String,
    id: String,
    revision_id: String,
    pub revision_text: String,
}

fn create_db() -> Result<Connection> {
    let conn = Connection::open("wiki.db");

    match &conn {
        Ok(conn) => {
            println!("Connected to DB");
            conn.execute(
                "CREATE TABLE IF NOT EXISTS pages (
                  title TEXT NOT NULL,
                  id TEXT NOT NULL,
                  revision_id TEXT NOT NULL,
                  revision_text TEXT NOT NULL
                  )",
                params![],
            )?;
        }
        Err(_err) => println!("Error connecting to DB"),
    }
    conn
}

fn get_conn() -> Connection {
    create_db().unwrap()
}

pub fn insert_page(page: &Page) -> () {
    let conn = get_conn();

    let res: Result<usize> = conn.execute(
        "INSERT INTO pages (title, id, revision_id, revision_text)
         VALUES (?1, ?2, ?3, ?4)",
        params![page.title, page.id, page.revision.id, page.revision.text],
    );

    match res {
        Ok(_num) => println!("Inserted page: {}", page.title),
        Err(_err) => println!("Error inserting page: {}, {}", page.title, _err),
    }
}

pub fn get_page(title: &str) -> Option<PageRow> {
    let conn = get_conn();

    let mut stmt = conn
        .prepare("SELECT title, id, revision_id, revision_text FROM pages WHERE title = ?1")
        .unwrap();
    let page_iter = stmt
        .query_map(params![title], |row| {
            Ok(PageRow {
                title: row.get(0)?,
                id: row.get(1)?,
                revision_id: row.get(2)?,
                revision_text: row.get(3)?,
            })
        })
        .unwrap();

    for page in page_iter {
        dbg!(&page);
        return Some(page.unwrap());
    }
    None
}
