use dioxus::prelude::*;
use dioxus_html_macro::html;

#[derive(Clone, PartialEq)]
struct Task {
    id: u32,
    title: String,
    done: bool,
}

#[component]
pub fn TaskList() -> Element {
    let tasks = use_signal(|| vec![
        Task { id: 1, title: "Настроить Dioxus".into(), done: true },
        Task { id: 2, title: "Перевести шаблоны на html!".into(), done: false },
    ]);

    html! {
        <ul class="task-list">
            {tasks.read().iter().map(|task| html! {
                <li key="{task.id}" class={if task.done { "done" } else { "" }}>
                    <span>"{task.title}"</span>
                </li>
            })}
        </ul>
    }
}
