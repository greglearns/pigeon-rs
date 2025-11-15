use anyhow::Result;
use connectorx::{
    destinations::arrow2::Arrow2Destination,
    prelude::{Dispatcher, SQLiteArrow2Transport},
    sources::sqlite::SQLiteSource,
    sql::CXQuery,
};
use polars::frame::DataFrame;

pub fn query_sqlite(conn: &str, query: &str) -> Result<DataFrame, anyhow::Error> {
    let source = SQLiteSource::new(conn, 3)?;
    let mut destination = Arrow2Destination::new();
    let queries = &[CXQuery::naked(query)];
    let dispatcher =
        Dispatcher::<_, _, SQLiteArrow2Transport>::new(source, &mut destination, queries, None);
    dispatcher.run()?;
    let df = destination.polars()?;

    Ok(df)
}
