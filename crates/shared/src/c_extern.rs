use std::ptr::null;
use crate::{TodoAppError, TodoList};
use std::sync::OnceLock;
use tokio::runtime::Runtime;

#[repr(C)]
struct TodoItem {
    id: i64,
    title: *const u8,
    title_len: usize,
    description: *const u8,
    description_len: usize,
    done: bool,
}

static RUNTIME: OnceLock<Runtime> = OnceLock::new();

fn runtime() -> &'static Runtime {
    RUNTIME.get_or_init(|| Runtime::new().expect("Failed to build tokio runtime"))
}

#[unsafe(no_mangle)]
extern "C" fn todo_initialize(error: *mut u8, error_len: *mut usize) -> i32 {
    if error.is_null() || error_len.is_null(){
        return -1;
    }
    match runtime().block_on(TodoList::initialize()) {
        Ok(_) => 0,
        Err(e) => match e {
            TodoAppError::SqlxError(e) => {
                let message = e.to_string();
                let bytes = message.as_bytes();
                let capacity = unsafe{*error_len};
                let count = bytes.len().min(capacity);
                unsafe{
                    std::ptr::copy_nonoverlapping(bytes.as_ptr(), error, count);
                    *error_len = count;
                }
                1
            }
            TodoAppError::IOError(e) => {
                let message = e.to_string();
                let bytes = message.as_bytes();
                let capacity = unsafe{*error_len};
                let count = bytes.len().min(capacity);
                unsafe{
                    std::ptr::copy_nonoverlapping(bytes.as_ptr(), error, count);
                    *error_len = count;
                }
                2
            }
        },
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn get_list(size: *mut usize, error: *mut u8, error_len: *mut usize) -> *const TodoItem {
    if error.is_null() || error_len.is_null(){
        return null();
    }
    if size.is_null(){
        let message = "Size parameter is null";
        let bytes = message.as_bytes();
        let capacity = unsafe{*error_len};
        let count = bytes.len().min(capacity);
        unsafe{
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), error, count);
            *error_len = count;
        }

        return null();
    }
    let rlist = match runtime()
        .block_on(TodoList::get()){
        Ok(list) => list,
        Err(e) => {
            let message = e.to_string();
            let bytes = message.as_bytes();
            let capacity = unsafe{*error_len};
            let count = bytes.len().min(capacity);
            unsafe{
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), error, count);
                *error_len = count;
            }
            return null();
        }
    };
    let clist = rlist
        .list
        .iter()
        .map(|item| {
            let (title, title_len) = string_to_pointer(item.title.clone());
            let (description, description_len) = string_to_pointer(item.description.clone());

            TodoItem {
                id: item.id,
                done: item.done,
                title,
                title_len,
                description,
                description_len,
            }
        })
        .collect::<Vec<TodoItem>>();
    let clist = clist.into_boxed_slice();
    unsafe {
        *size = clist.len();
    }
    Box::into_raw(clist) as *const TodoItem
}

fn string_to_pointer(str: impl Into<String>) -> (*const u8, usize) {
    let str = str.into();
    let str = str.into_boxed_str();
    let size = str.len();
    let raw = Box::into_raw(str);
    (raw as *const u8, size)
}
