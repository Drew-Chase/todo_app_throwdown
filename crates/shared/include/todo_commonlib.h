#pragma once
#include <vector>
#include <cstdint>
#include <string>

class TodoItem {
public:
    std::int64_t id;
    std::string title;
    std::string description;
    bool done;

    TodoItem(
        const std::int64_t id,
        std::string title,
        std::string description,
        const bool done
    ) : id(id),
        title(std::move(title)),
        description(std::move(description)),
        done(done) {
    }

    static TodoItem get(std::int64_t id);

    void delete_item();

    void insert();

    void mark_done();

    void mark_undone();
};

class TodoList {
public:
    static void initialize();

    static std::vector<TodoItem> get();

    static std::int64_t insert(std::string title, std::string description);

    static void mark_done(std::int64_t id);

    static void mark_undone(std::int64_t id);

    static void delete_item(std::int64_t id);
};
