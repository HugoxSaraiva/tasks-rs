use crate::domain::Task;

pub fn format_string_to_constraint(text: &str, max_len: usize) -> &str {
    let text_len = text.len();
    if text_len <= max_len {
        return text;
    }
    &text[0..max_len]
}

pub fn print_task_headers() {
    println!(
        "|{:^5}|{:^40}|{:^20}|{:^25}|{:^15}|",
        "ID", "Description", "Scope", "Created at", "Completed"
    );
}

pub fn print_task(task: &Task) {
    println!(
        "|{:^5}|{:^40}|{:^20}|{:^25}|{:^15}|",
        format_string_to_constraint(&task.id.to_string(), 5),
        format_string_to_constraint(&task.description, 40),
        format_string_to_constraint(task.scope.as_ref().map_or("None", |s| s.as_ref()), 20),
        format_string_to_constraint(&task.created_at.format("%Y-%m-%d %H:%M:%S").to_string(), 25),
        format_string_to_constraint(task.completed_at.map_or(" ", |_| "x"), 15)
    );
}
