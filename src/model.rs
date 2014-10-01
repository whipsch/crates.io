use pg::PostgresRow;

use db::Connection;
use util::{CargoResult, Require, CargoError};
use util::errors::NotFound;

pub trait Model {
    fn from_row(row: &PostgresRow) -> Self;
    fn table_name(_: Option<Self>) -> &'static str;

    fn find(conn: &Connection, id: i32) -> CargoResult<Self> {
        let sql = format!("SELECT * FROM {} WHERE id = $1",
                          Model::table_name(None::<Self>));
        let stmt = try!(conn.prepare(sql.as_slice()));
        let mut rows = try!(stmt.query(&[&id]));
        let row = try!(rows.next().require(|| NotFound.box_error()));
        Ok(Model::from_row(&row))
    }
}