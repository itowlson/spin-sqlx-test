use std::fmt::Display;

// use sqlx::Row;
use sqlx::ColumnIndex;

mod convert;

impl ColumnIndex<SpinSqliteRow> for usize {
    fn index(&self, container: &SpinSqliteRow) -> Result<usize, sqlx::Error> {
        if *self < container.inner.values.len() {
            Ok(*self)
        } else {
            Err(sqlx::Error::ColumnIndexOutOfBounds { index: *self, len: container.inner.values.len() })
        }
    }
}

impl ColumnIndex<SpinSqliteRow> for &str {
    fn index(&self, container: &SpinSqliteRow) -> Result<usize, sqlx::Error> {
        container.columns.iter().position(|c| c == self)
            .ok_or_else(|| sqlx::Error::ColumnNotFound(self.to_string()))
    }
}


#[derive(Debug)]
pub struct SqlxConnection(spin_sdk::sqlite::Connection);

impl SqlxConnection {
    pub fn new(conn: spin_sdk::sqlite::Connection) -> Self {
        Self(conn)
    }

    pub fn open(label: &str) -> anyhow::Result<Self> {
        Ok(Self(spin_sdk::sqlite::Connection::open(label)?))
    }

    pub fn open_default() -> anyhow::Result<Self> {
        Ok(Self(spin_sdk::sqlite::Connection::open_default()?))
    }
}

#[derive(Clone, Debug)]
pub struct SqlxConnectionOptions {
    label: String,
}

impl sqlx::Connection for SqlxConnection {
    type Database = SqlxConnection;

    type Options = SqlxConnectionOptions;

    fn close(self) -> BoxFuture<'static, Result<(), sqlx::Error>> {
        Box::pin(async move { Ok(()) })
    }
    fn close_hard(self) -> BoxFuture<'static, Result<(), sqlx::Error>> {
        Box::pin(async move { Ok(()) })
    }

    fn ping(&mut self) -> BoxFuture<'_, Result<(), sqlx::Error>> {
        Box::pin(async move { Ok(()) })
    }

    fn begin(&mut self) -> BoxFuture<'_, Result<sqlx::Transaction<'_, Self::Database>, sqlx::Error>>
    where
        Self: Sized {
        todo!()
    }

    fn shrink_buffers(&mut self) {
    }
    fn flush(&mut self) -> BoxFuture<'_, Result<(), sqlx::Error>> {
        Box::pin(async move { Ok(()) })
    }

    fn should_flush(&self) -> bool { false }
}

impl sqlx::Database for SqlxConnection {
    type Connection = SqlxConnection;

    type TransactionManager = SqlxConnection;

    type Row = SpinSqliteRow;

    type QueryResult = SpinSqliteQR;

    type Column = SpinSqliteColumn;

    type TypeInfo = SpinSqliteTypeInfo;

    type Value = SpinSqliteValue;

    const NAME: &'static str = "Spin SQLite";

    const URL_SCHEMES: &'static [&'static str] = &["spin-sqlite"];
}

pub struct SpinSqliteRow {
    columns: std::sync::Arc<Vec<String>>,
    inner: spin_sdk::sqlite::RowResult,
}
#[derive(Default)]
pub struct SpinSqliteQR {
    // inner: Option<spin_sdk::sqlite::QueryResult>,  // Option because we can't construct a default one
}

#[derive(Debug)]
pub struct SpinSqliteColumn {

}
#[derive(Clone, Debug, PartialEq)]
pub enum SpinSqliteTypeInfo {
    Int,
    Blob,
    Real,
    Text,
    Null,
}

pub struct SpinSqliteValue {
    // inner: spin_sdk::sqlite::Value,
}

impl sqlx::Row for SpinSqliteRow {
    type Database = SqlxConnection;

    fn columns(&self) -> &[<Self::Database as sqlx::Database>::Column] {
        todo!()
    }

    fn try_get_raw<I>(
        &self,
        index: I,
    ) -> Result<<Self::Database as sqlx::database::HasValueRef<'_>>::ValueRef, sqlx::Error>
    where
        I: sqlx::ColumnIndex<Self> {
        let uindex = index.index(&self)?;

        if uindex >= self.inner.values.len() {
            return Err(sqlx::Error::ColumnIndexOutOfBounds { index: uindex, len: self.inner.values.len() });
        }

        let val = &self.inner.values[uindex];
        Ok(SpinSqliteValueRef { inner: val.clone() })
    }
}

impl Extend<SpinSqliteQR> for SpinSqliteQR {
    fn extend<T: IntoIterator<Item = SpinSqliteQR>>(&mut self, _iter: T) {
    }
}

impl SpinSqliteQR {
    // fn rows(&self) -> Vec<spin_sdk::sqlite::Row<'_>> {
    //     match &self.inner {
    //         Some(qr) => qr.rows().collect(),
    //         None => vec![],
    //     }
    // }

    // Looks like SQLite doesn't give us this back?
    // pub fn rows_affected(&self) -> u64 {
    //     match &self.inner {
    //         None => panic!("what the heck, rows_affected called on empty"),
    //         Some(qr) => match &qr.rows[0].values[0] {
    //             &spin_sdk::sqlite::Value::Integer(n) => n.try_into().unwrap(),
    //             _ => panic!("NAN NAN NAN"),
    //         }
    //     }
    // }
}

impl sqlx::Column for SpinSqliteColumn {
    type Database = SqlxConnection;

    fn ordinal(&self) -> usize {
        todo!()
    }

    fn name(&self) -> &str {
        todo!()
    }

    fn type_info(&self) -> &<Self::Database as sqlx::Database>::TypeInfo {
        todo!()
    }
}

impl Display for SpinSqliteTypeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use sqlx::TypeInfo;
        f.write_str(self.name())
    }
}
impl sqlx::TypeInfo for SpinSqliteTypeInfo {
    fn is_null(&self) -> bool {
        *self == Self::Null
    }

    fn name(&self) -> &str {
        match self {
            Self::Blob => "BINARY",
            Self::Int => "INT",
            Self::Null => "NULL",
            Self::Real => "REAL",
            Self::Text => "TEXT",
        }
    }
}

impl sqlx::Value for SpinSqliteValue {
    type Database = SqlxConnection;

    fn as_ref(&self) -> <Self::Database as sqlx::database::HasValueRef<'_>>::ValueRef {
        todo!()
    }

    fn type_info(&self) -> std::borrow::Cow<'_, <Self::Database as sqlx::Database>::TypeInfo> {
        todo!()
    }

    fn is_null(&self) -> bool {
        todo!()
    }
}

impl<'q> sqlx::database::HasArguments<'q> for SqlxConnection {
    type Database = SqlxConnection;

    type Arguments = SpinSqliteArgs;

    type ArgumentBuffer = Vec<spin_sdk::sqlite::Value>;
}

#[derive(Default)]
pub struct SpinSqliteArgs {
    inner: Vec<spin_sdk::sqlite::Value>,
}

impl SpinSqliteArgs {
    fn as_slice(&self) -> &[spin_sdk::sqlite::Value] {
        &self.inner
    }
}

impl<'q> sqlx::Arguments<'q> for SpinSqliteArgs {
    type Database = SqlxConnection;

    fn reserve(&mut self, additional: usize, size: usize) {
        todo!()
    }

    fn add<T>(&mut self, value: T)
    where
        T: 'q + Send + sqlx::Encode<'q, Self::Database> + sqlx::Type<Self::Database> {
        let _ = value.encode_by_ref(&mut self.inner);
    }
}

impl<'q> sqlx::IntoArguments<'q, SqlxConnection> for SpinSqliteArgs {
    fn into_arguments(self) -> <SqlxConnection as sqlx::database::HasArguments<'q>>::Arguments {
        self
    }
}

impl<'q> sqlx::database::HasStatement<'q> for SqlxConnection {
    type Database = SqlxConnection;

    type Statement = SpinSqliteStmt;
}

#[derive(Default)]
pub struct SpinSqliteStmt {

}

impl<'q> sqlx::Statement<'q> for SpinSqliteStmt {
    type Database = SqlxConnection;

    fn to_owned(&self) -> <Self::Database as sqlx::database::HasStatement<'static>>::Statement {
        todo!()
    }

    fn sql(&self) -> &str {
        todo!()
    }

    fn parameters(&self) -> Option<either::Either<&[<Self::Database as sqlx::Database>::TypeInfo], usize>> {
        todo!()
    }

    fn columns(&self) -> &[<Self::Database as sqlx::Database>::Column] {
        todo!()
    }

    fn query(&self) -> sqlx::query::Query<'_, Self::Database, <Self::Database as sqlx::database::HasArguments<'_>>::Arguments> {
        todo!()
    }

    fn query_with<'s, A>(&'s self, arguments: A) -> sqlx::query::Query<'s, Self::Database, A>
    where
        A: sqlx::IntoArguments<'s, Self::Database> {
        todo!()
    }

    fn query_as<O>(
        &self,
    ) -> sqlx::query::QueryAs<'_, Self::Database, O, <Self::Database as sqlx::database::HasArguments<'_>>::Arguments>
    where
        O: for<'r> sqlx::FromRow<'r, <Self::Database as sqlx::Database>::Row> {
        todo!()
    }

    fn query_as_with<'s, O, A>(&'s self, arguments: A) -> sqlx::query::QueryAs<'s, Self::Database, O, A>
    where
        O: for<'r> sqlx::FromRow<'r, <Self::Database as sqlx::Database>::Row>,
        A: sqlx::IntoArguments<'s, Self::Database> {
        todo!()
    }

    fn query_scalar<O>(
        &self,
    ) -> sqlx::query::QueryScalar<'_, Self::Database, O, <Self::Database as sqlx::database::HasArguments<'_>>::Arguments>
    where
        (O,): for<'r> sqlx::FromRow<'r, <Self::Database as sqlx::Database>::Row> {
        todo!()
    }

    fn query_scalar_with<'s, O, A>(&'s self, arguments: A) -> sqlx::query::QueryScalar<'s, Self::Database, O, A>
    where
        (O,): for<'r> sqlx::FromRow<'r, <Self::Database as sqlx::Database>::Row>,
        A: sqlx::IntoArguments<'s, Self::Database> {
        todo!()
    }
}

impl<'q> sqlx::database::HasValueRef<'q> for SqlxConnection {
    type Database = SqlxConnection;

    type ValueRef = SpinSqliteValueRef;
}

pub struct SpinSqliteValueRef {
    inner: spin_sdk::sqlite::Value,
}

impl<'q> sqlx::ValueRef<'q> for SpinSqliteValueRef {
    type Database = SqlxConnection;

    fn to_owned(&self) -> <Self::Database as sqlx::Database>::Value {
        todo!()
    }

    fn type_info(&self) -> std::borrow::Cow<'_, <Self::Database as sqlx::Database>::TypeInfo> {
        let type_info = match &self.inner {
            spin_sdk::sqlite::Value::Null => SpinSqliteTypeInfo::Null,
            spin_sdk::sqlite::Value::Integer(_) => SpinSqliteTypeInfo::Int,
            spin_sdk::sqlite::Value::Blob(_) => SpinSqliteTypeInfo::Blob,
            spin_sdk::sqlite::Value::Real(_) => SpinSqliteTypeInfo::Real,
            spin_sdk::sqlite::Value::Text(_) => SpinSqliteTypeInfo::Text,
        };
        std::borrow::Cow::Owned(type_info)
    }

    fn is_null(&self) -> bool {
        matches!(&self.inner, spin_sdk::sqlite::Value::Null)
    }
}

impl sqlx::TransactionManager for SqlxConnection {
    type Database = SqlxConnection;

    fn begin(
        conn: &mut <Self::Database as sqlx::Database>::Connection,
    ) -> BoxFuture<'_, Result<(), sqlx::Error>> {
        todo!()
    }

    fn commit(
        conn: &mut <Self::Database as sqlx::Database>::Connection,
    ) -> BoxFuture<'_, Result<(), sqlx::Error>> {
        todo!()
    }

    fn rollback(
        conn: &mut <Self::Database as sqlx::Database>::Connection,
    ) -> BoxFuture<'_, Result<(), sqlx::Error>> {
        todo!()
    }

    fn start_rollback(conn: &mut <Self::Database as sqlx::Database>::Connection) {
        todo!()
    }
}

impl std::str::FromStr for SqlxConnectionOptions {
    type Err = sqlx::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { label: s.to_owned() })
    }
}

use log::LevelFilter;

impl sqlx::ConnectOptions for SqlxConnectionOptions {
    type Connection = SqlxConnection;

    fn from_url(url: &url::Url) -> Result<Self, sqlx::Error> {
        let label = url.host().unwrap().to_string();  // TODO: figure out the stupid error type
        Ok(Self { label })
    }

    fn connect(&self) -> BoxFuture<'_, Result<Self::Connection, sqlx::Error>>
    where
        Self::Connection: Sized {
            Box::pin(async move {
                spin_sdk::sqlite::Connection::open(&self.label)
                    .map(|conn| SqlxConnection(conn))
                    .map_err(|e| sqlx::Error::AnyDriverError(Box::new(e)))
            })
    }

    fn log_statements(self, _level: LevelFilter) -> Self {
        self
    }

    fn log_slow_statements(self, _level: LevelFilter, _duration: std::time::Duration) -> Self {
        self
    }
}







use futures_core::future::BoxFuture;
use futures_core::stream::BoxStream;

impl<'c> sqlx::Executor<'c> for &'c SqlxConnection {
    type Database = SqlxConnection;

    fn fetch_many<'e, 'q: 'e, E: 'q>(
        self,
        mut query: E,
    ) -> BoxStream<
        'e,
        Result<
            sqlx::Either<<Self::Database as sqlx::Database>::QueryResult, <Self::Database as sqlx::Database>::Row>,
            sqlx::Error,
        >,
    >
    where
        'c: 'e,
        E: sqlx::Execute<'q, Self::Database> {

        println!("FETCH-MANYing {}", query.sql());
        // The args-exec dance needs to go on the SqlxConnection object
        let args = query.take_arguments().unwrap_or_default();
        let rs = self.0.execute(query.sql(), args.as_slice()).unwrap();

        // Okay this CANNOT return a QueryResult because fetch will filtermap any
        // Either::Lefts away because reasons.  We have to get the rows.

        // let qr = SpinSqliteQR { inner: Some(rs) };
        // let res = Ok(sqlx::Either::Left(qr));
        // Box::pin(futures::stream::once(async { res }))

        let columns = std::sync::Arc::new(rs.columns.clone());
        let rows = rs.rows.into_iter()
            .map(move |r| Ok(sqlx::Either::Right(SpinSqliteRow { columns: columns.clone(), inner: r })));
        Box::pin(futures::stream::iter(rows))
    }

    fn execute<'e, 'q: 'e, E: 'q>(
            self,
            mut query: E,
        ) -> BoxFuture<'e, Result<<Self::Database as sqlx::Database>::QueryResult, sqlx::Error>>
        where
            'c: 'e,
            E: sqlx::Execute<'q, Self::Database>, {
        println!("EXECing {}", query.sql());
        let args = query.take_arguments().unwrap_or_default();
        let rs = self.0.execute(query.sql(), args.as_slice()).unwrap();

        let qr = SpinSqliteQR { /*inner: Some(rs)*/ };
        let res = Ok(qr);
        Box::pin(async { res })
    }

    fn fetch_optional<'e, 'q: 'e, E: 'q>(
        self,
        mut query: E,
    ) -> BoxFuture<'e, Result<Option<<Self::Database as sqlx::Database>::Row>, sqlx::Error>>
    where
        'c: 'e,
        E: sqlx::Execute<'q, Self::Database> {
        println!("FETCH-OPTIONALing {}", query.sql());
        let args = query.take_arguments().unwrap_or_default();
        let rs = self.0.execute(query.sql(), args.as_slice()).unwrap();

        let columns = std::sync::Arc::new(rs.columns.clone());
        let row = rs.rows.into_iter()
            .map(move |r| SpinSqliteRow { columns: columns.clone(), inner: r })
            .next();

        Box::pin(async { Ok(row) })
    }

    fn prepare_with<'e, 'q: 'e>(
        self,
        sql: &'q str,
        parameters: &'e [<Self::Database as sqlx::Database>::TypeInfo],
    ) -> BoxFuture<'e, Result<<Self::Database as sqlx::database::HasStatement<'q>>::Statement, sqlx::Error>>
    where
        'c: 'e {
        todo!()
    }

    fn describe<'e, 'q: 'e>(
        self,
        sql: &'q str,
    ) -> BoxFuture<'e, Result<sqlx::Describe<Self::Database>, sqlx::Error>>
    where
        'c: 'e {
            todo!()
        }


    // fn describe<'q>(self, _: &'q str) -> std::pin::Pin<Box<
    //     (dyn futures_core::Future<Output = Result<sqlx::Describe<<Self as sqlx::Executor<'c>>::Database>, sqlx::Error>> + std::marker::Send + 'e)>> { todo!() }`: `fn describe(self, _: &'q str) -> Pin<Box<(dyn futures_core::Future<Output = Result<Describe<<Self as Executor<'c>>::Database>, sqlx::Error>> + std::marker::Send + 'e)>> { todo!() }
}

