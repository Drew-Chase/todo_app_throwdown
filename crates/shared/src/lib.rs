use sqlx::sqlite::SqliteJournalMode;
use sqlx::{AssertSqlSafe, ConnectOptions, Connection, Executor, SqlSafeStr, Statement};

#[derive(thiserror::Error, Debug)]
pub enum TodoAppError {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}

#[derive(sqlx::FromRow, Debug)]
pub struct TodoItem {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub done: bool,
}
pub struct TodoList {
    pub list: Vec<TodoItem>,
}

const TODO_TABLE_NAME: &str = "todos";

impl TodoList {
    pub async fn initialize() -> Result<(), TodoAppError> {
        let mut connection = Self::open().await?;
        connection
            .execute(AssertSqlSafe(format!(
                "
CREATE TABLE IF NOT EXISTS `{TODO_TABLE_NAME}`
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    title       TEXT NOT NULL,
    description TEXT NOT NULL,
    done        INTEGER DEFAULT 0
)"
            )))
            .await?;

        Ok(())
    }
    pub async fn get() -> Result<Self, TodoAppError> {
        let mut connection = Self::open().await?;

        let items: Vec<TodoItem> = sqlx::query_as::<_, TodoItem>(AssertSqlSafe(format!(
            "select * from {TODO_TABLE_NAME}"
        )))
        .fetch_all(&mut connection)
        .await?;
        connection.close().await?;

        Ok(Self { list: items })
    }

    pub async fn insert(title: String, description: String) -> Result<i64, TodoAppError> {
        let mut connection = Self::open().await?;
        let stmt = connection
            .prepare(
                AssertSqlSafe(format!(
                    "insert into `{TODO_TABLE_NAME}` (title, description) values (?,?)"
                ))
                .into_sql_str(),
            )
            .await?;

        let id = stmt
            .query()
            .bind(title)
            .bind(description)
            .execute(&mut connection)
            .await?
            .last_insert_rowid();
        connection.close().await?;

        Ok(id)
    }

    pub async fn mark_done(id: i64) -> Result<(), TodoAppError> {
        let mut connection = Self::open().await?;
        let stmt = connection
            .prepare(
                AssertSqlSafe(format!(
                    "update `{TODO_TABLE_NAME}` set done = 1 where id = ?"
                ))
                .into_sql_str(),
            )
            .await?;
        stmt.query().bind(id).execute(&mut connection).await?;
        connection.close().await?;

        Ok(())
    }

    pub async fn mark_undone(id: i64) -> Result<(), TodoAppError> {
        let mut connection = Self::open().await?;
        let stmt = connection
            .prepare(
                AssertSqlSafe(format!(
                    "update `{TODO_TABLE_NAME}` set done = 0 where id = ?"
                ))
                .into_sql_str(),
            )
            .await?;
        stmt.query().bind(id).execute(&mut connection).await?;
        connection.close().await?;

        Ok(())
    }

    async fn open() -> Result<sqlx::sqlite::SqliteConnection, TodoAppError> {
        let options = sqlx::sqlite::SqliteConnectOptions::new()
            .filename("todo.db")
            .journal_mode(SqliteJournalMode::Wal)
            .disable_statement_logging()
            .create_if_missing(true);
        Ok(options.connect().await?)
    }
}

impl TodoItem {
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> TodoItem {
        TodoItem {
            id: -1,
            title: title.into(),
            description: description.into(),
            done: false,
        }
    }
    pub async fn insert(&mut self) -> Result<(), TodoAppError> {
        self.id = TodoList::insert(self.title.clone(), self.description.clone()).await?;
        Ok(())
    }
    pub async fn mark_done(&mut self) -> Result<(), TodoAppError> {
        self.done = true;
        TodoList::mark_done(self.id).await
    }
    pub async fn mark_undone(&mut self) -> Result<(), TodoAppError> {
        self.done = false;
        TodoList::mark_undone(self.id).await
    }
}

#[cfg(feature = "tests")]
#[cfg(test)]
mod test {
    use super::*;
    #[tokio::test]
    async fn initialize_should_work() {
        TodoList::initialize().await.unwrap();
    }
    #[tokio::test]
    async fn insert_should_work() {
        TodoList::initialize().await.unwrap();
        let mut item = TodoItem::new("Insert Item", "This is a demo item!");
        item.insert().await.unwrap();
    }
    #[tokio::test]
    async fn mark_done_should_work() {
        TodoList::initialize().await.unwrap();
        let mut item = TodoItem::new("Mark Done Item", "This is a demo item!");
        item.insert().await.unwrap();
        item.mark_done().await.unwrap();
    }
    #[tokio::test]
    async fn mark_undone_should_work() {
        TodoList::initialize().await.unwrap();
        let mut item = TodoItem::new("Mark Undone Item", "This is a demo item!");
        item.insert().await.unwrap();
        item.mark_undone().await.unwrap();
    }
    #[tokio::test]
    async fn fetch_should_work() {
        TodoList::initialize().await.unwrap();
        let list = TodoList::get().await.unwrap();
        for item in list.list {
            println!("{:?}", item);
        }
    }

}
