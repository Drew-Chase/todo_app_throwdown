#include "todo_commonlib.h"
#include <cstdint>
#include <cstddef>
#include <stdexcept>
#include <string>


struct m_TodoItem {
    std::int64_t id;
    const char* title;
    const std::size_t title_len;
    const char* description;
    const std::size_t description_len;
    bool done;
};

extern "C" std::int32_t todo_initialize(std::uint8_t *error, std::size_t *error_len);

extern "C" const m_TodoItem *get_list(std::size_t *size, std::uint8_t *error, std::size_t *error_len);


void TodoList::initialize() {
    std::uint8_t err_buf[256] = {};
    std::size_t error_len = sizeof(err_buf);

    if (const int code = todo_initialize(err_buf, &error_len); code != 0) {
        if (code == -1) {
            throw std::runtime_error("The buffer and/or buffer size was null when sending a message to the todo library");
        }
        const std::string error_str(reinterpret_cast<const char *>(err_buf), error_len);
        throw std::runtime_error(error_str);
    }
}

std::vector<TodoItem> TodoList::get() {
    std::uint8_t err_buf[256] = {};
    std::size_t err_len = sizeof(err_buf);
    std::size_t list_size = 0;
    const m_TodoItem *items = get_list(&list_size, err_buf, &err_len);
    if (items == nullptr) {
        const std::string error_str(reinterpret_cast<const char *>(err_buf), err_len);
        throw std::runtime_error(error_str);
    }
    static_assert(sizeof(m_TodoItem) == 48);
    std::vector<TodoItem> list;
    list.reserve(list_size);
    for (std::size_t i = 0; i < list_size; ++i) {
        auto [id, title, title_len, description, description_len, done] = items[i];
        list.emplace_back(id, std::string(title, title_len), std::string(description, description_len), done);
    }
    return list;
}
