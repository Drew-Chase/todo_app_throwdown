use tauri::command;
use todo_commonlib::{TodoItem, TodoList};
use tracing::{debug, error};

#[command]
pub async fn get_list() -> Result<Vec<TodoItem>, String> {
    debug!("IPC get_list");
    let items = TodoList::get().await.map_err(|e| {
        error!("get_list failed: {e}");
        e.to_string()
    })?;
    Ok(items.list)
}

#[command]
pub async fn insert_item(title: String, description: String) -> Result<TodoItem, String> {
    debug!("IPC insert_item");
    let id = TodoList::insert(title, description).await.map_err(|e| {
        error!("insert_item: insert failed: {e}");
        e.to_string()
    })?;
    let item = TodoItem::get(id).await.map_err(|e| {
        error!("insert_item: fetch failed: {e}");
        e.to_string()
    })?;
    Ok(item)
}

#[command]
pub async fn mark_item_as_done(id: i64) -> Result<(), String> {
    debug!("IPC mark_item_as_done (id {id})");
    let mut item = TodoItem::get(id).await.map_err(|e| {
        error!("mark_item_as_done: fetch failed: {e}");
        e.to_string()
    })?;
    item.mark_done().await.map_err(|e| {
        error!("mark_item_as_done failed: {e}");
        e.to_string()
    })?;
    Ok(())
}

#[command]
pub async fn mark_item_as_undone(id: i64) -> Result<(), String> {
    debug!("IPC mark_item_as_undone (id {id})");
    let mut item = TodoItem::get(id).await.map_err(|e| {
        error!("mark_item_as_undone: fetch failed: {e}");
        e.to_string()
    })?;
    item.mark_undone().await.map_err(|e| {
        error!("mark_item_as_undone failed: {e}");
        e.to_string()
    })?;
    Ok(())
}

#[command]
pub async fn delete_item(id: i64) -> Result<(), String> {
    debug!("IPC delete_item (id {id})");
    TodoList::delete(id).await.map_err(|e| {
        error!("delete_item failed: {e}");
        e.to_string()
    })?;
    Ok(())
}

#[command]
pub async fn initialize() -> Result<(), String> {
    debug!("IPC initialize");
    TodoList::initialize().await.map_err(|e| {
        error!("initialize failed: {e}");
        e.to_string()
    })?;
    Ok(())
}

