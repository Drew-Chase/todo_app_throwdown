use tauri::command;
use todo_commonlib::{TodoItem, TodoList};

#[command]
pub async fn get_list() -> Result<Vec<TodoItem>, String> {
    let items = TodoList::get().await.map_err(|e| e.to_string())?;
    Ok(items.list)
}

#[command]
pub async fn insert_item(title: String, description: String) -> Result<TodoItem, String> {
    let id = TodoList::insert(title, description)
        .await
        .map_err(|e| e.to_string())?;
    let item = TodoItem::get(id).await.map_err(|e| e.to_string())?;
    Ok(item)
}

#[command]
pub async fn mark_item_as_done(id: i64) -> Result<(), String> {
    let mut item = TodoItem::get(id).await.map_err(|e| e.to_string())?;
    item.mark_done().await.map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
pub async fn mark_item_as_undone(id: i64) -> Result<(), String> {
    let mut item = TodoItem::get(id).await.map_err(|e| e.to_string())?;
    item.mark_undone().await.map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
pub async fn delete_item(id: i64) -> Result<(), String> {
    TodoList::delete(id).await.map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
pub async fn initialize() -> Result<(), String> {
    TodoList::initialize().await.map_err(|e| e.to_string())?;
    Ok(())
}

