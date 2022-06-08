use yew::prelude::*;


#[derive(Clone, PartialEq)]
struct TodoItem {
    id: usize,
    text: String,
    done: bool
}

#[derive(Clone, Properties, PartialEq)]
struct TodoItemProps {
    id: usize,
    text: String,
    done: bool
}

#[derive(Clone, Properties, PartialEq)]
struct TodoListProps {
    todos: Vec<TodoItem>
}

#[function_component(Todo)]
fn todo(TodoItemProps { id, text, done }: &TodoItemProps) -> Html {
    html! {
        // impl
    }
}

#[function_component(TodoList)]
fn todo_list(TodoListProps { todos }: &TodoListProps) -> Html {
    html! {
        // impl
    }
}


#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <section class="todoapp">
                <header class="header">
                    <h1>{ "todos" }</h1>
                    <input autofocus={true} autocomplete="off" placeholder="What needs to be done?" class="new-todo" />
                </header>

                <section class="main">
                    <input id="toggle-all" class="toggle-all" type="checkbox" />
                    <label for="toggle-all">{"Mark all as complete"}</label>
                    <ul class="todo-list">
                        <li class="todo">
                            <div class="view">
                                <input class="toggle" type="checkbox" />
                                <label></label>
                                <button class="destroy"></button>
                            </div>
                            <input class="edit" type="text" />
                        </li>
                    </ul>
                </section>

                <footer class="footer">
                    <span class="todo-count"><strong>{"left"}</strong></span>
                    <ul class="filters">
                        <li><a href="">{"All"}</a></li>
                        <li><a href="">{"Active"}</a></li>
                        <li><a href="">{"Completed"}</a></li>
                    </ul>
                    <button class="clear-completed">{"Clear Completed"}</button>
                </footer>
            
            </section>

            <footer class="info">
                <p>{"Double-click to edit a todo"}</p>
                <p>{"Created by"} <a href="">{"negativefix"}</a></p>
                <p>{"Part of"} <a href="">{"TodoMVC"}</a></p>
		    </footer>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}

