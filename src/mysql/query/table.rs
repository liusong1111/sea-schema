use std::rc::Rc;
#[cfg(feature="sqlx-mysql")]
use sqlx::{Row, mysql::MySqlRow};
use sea_query::{Expr, Iden, Order, Query, SelectStatement};
use super::{InformationSchema, SchemaQuery};

#[derive(Debug, sea_query::Iden)]
/// Ref: https://dev.mysql.com/doc/refman/8.0/en/information-schema-tables-table.html
pub enum TablesFields {
    TableCatalog,
    TableSchema,
    TableName,
    TableType,
    Engine,
    Version,
    RowFormat,
    TableRows,
    AvgRowLength,
    DataLength,
    MaxDataLength,
    IndexLength,
    DataFree,
    AutoIncrement,
    CreateTime,
    UpdateTime,
    CheckTime,
    TableCollation,
    Checksum,
    CreateOptions,
    TableComment,
}

#[derive(Debug, sea_query::Iden)]
pub enum TableType {
    #[iden = "BASE TABLE"]
    BaseTable,
    View,
    #[iden = "SYSTEM VIEW"]
    SystemView,
}

#[derive(Debug)]
pub struct TableQueryResult {
    pub table_name: String,
    pub engine: String,
    pub auto_increment: i32,
    pub table_collation: String,
    pub table_comment: String,
    pub create_options: String,
}

impl SchemaQuery {
    pub fn query_tables(&self, schema: Rc<dyn Iden>) -> SelectStatement {
        Query::select()
            .columns(vec![
                TablesFields::TableName,
                TablesFields::Engine,
                TablesFields::AutoIncrement,
                TablesFields::TableCollation,
                TablesFields::TableComment,
                TablesFields::CreateOptions,
            ])
            .from((InformationSchema::Schema, InformationSchema::Tables))
            .and_where(Expr::col(TablesFields::TableSchema).eq(schema.to_string()))
            .and_where(Expr::col(TablesFields::TableType).eq(TableType::BaseTable.to_string()))
            .order_by(TablesFields::TableName, Order::Asc)
            .take()
    }
}

#[cfg(feature="sqlx-mysql")]
impl From<&MySqlRow> for TableQueryResult {
    fn from(row: &MySqlRow) -> Self {
        Self {
            table_name: row.get(0),
            engine: row.get(1),
            auto_increment: row.get(2),
            table_collation: row.get(3),
            table_comment: row.get(4),
            create_options: row.get(5),
        }
    }
}