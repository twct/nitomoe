use diesel::dsl::insert_into;
use diesel::{PgConnection, QueryResult, RunQueryDsl};

use crate::models::board::Board;
use crate::schema::boards::dsl::*;

pub fn insert_board(conn: &mut PgConnection, board: Board) -> QueryResult<Board> {
    insert_into(boards).values(&board).get_result(conn)
}
