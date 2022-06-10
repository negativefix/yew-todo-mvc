use std::vec;

use wasm_bindgen::JsCast;
use yew::prelude::*;
use web_sys::HtmlInputElement;


#[derive(Clone, PartialEq)]
struct Todo {
    id: usize,
    text: String,
    completed: bool
}

#[derive(Clone, Properties, PartialEq)]
struct TodoItemProps {
    todo: Todo,
    editing: bool,
    on_toggle: Callback<usize>,
    on_destroy: Callback<usize>,
}

#[function_component(TodoItem)]
fn todo(TodoItemProps { todo, editing, on_toggle, on_destroy }: &TodoItemProps) -> Html {

    let onchange = {
        let on_toggle = on_toggle.clone();
        let todo_id = todo.id.clone();
        Callback::from(move |e: Event| {
            e.prevent_default();
            on_toggle.emit(todo_id);
        })
    };

    let onclick = {
        let on_destroy = on_destroy.clone();
        let todo_id = todo.id.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            on_destroy.emit(todo_id);
        })
    };

    html! {
        <li class={classes!(
            "todo",
            editing.then(|| Some("editing")),
            todo.completed.then(|| Some("completed")),
            )}>
            <div class="view">
                <input class="toggle" type="checkbox" checked={todo.completed} {onchange}/>
                <label>{todo.text.clone()}</label>
                <button class="destroy" {onclick}></button>
            </div>
            <input class="edit" type="text" />
        </li>
    }
}

#[derive(PartialEq, Properties, Default)]
struct TodoAppProps;

enum TodoAppMsg {
    Add,
    Remove(usize),
    Complete(usize),
    MarkAllComplete(bool),
    ClearCompleted(),
    EditNewTodo(String),
}

struct TodoApp {
    todos: Vec<Todo>,
    new_todo: String,
}


impl Component for TodoApp {
    type Message = TodoAppMsg;
    type Properties = TodoAppProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { 
            todos:  vec![],
            new_todo: String::new(),
        }
    }


    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TodoAppMsg::EditNewTodo(text) =>  { self.new_todo = text.clone(); true },
            TodoAppMsg::Add => { 
                let todo = Todo {
                    id: self.todos.len() + 1,
                    text: self.new_todo.clone(),
                    completed: false
                };
                self.todos.push(todo); 
                self.new_todo = "".to_string();
                true 
            },
            TodoAppMsg::Complete(id) =>  {
                
                let result = self
                    .todos
                    .iter_mut()
                    .find(|todo| todo.id == id);

                if let Some(todo) = result {
                    todo.completed = !todo.completed;
                    true
                } else {
                    false
                }
                
            },
            TodoAppMsg::Remove(id) => {
                let index = self.todos.iter().position(|todo| todo.id == id);
                if let Some(index) = index {
                    self.todos.remove(index);
                    true
                } else {
                    false
                }
            },
            TodoAppMsg::ClearCompleted() => true,
            TodoAppMsg::MarkAllComplete(marked) => {
                if marked {
                    self
                        .todos
                        .iter_mut()
                        .for_each(|todo| todo.completed = true);
                        true
                } else {
                    self
                        .todos
                        .iter_mut()
                        .for_each(|todo| todo.completed = false);
                        true
                }
            }
        }
    }


    fn view(&self, ctx: &Context<Self>) -> Html {
        let todos = self.todos.clone();
        let todos_active: Vec<Todo> = self.todos.clone().into_iter().filter(|todo| !todo.completed).collect();
        let has_completed_todos = todos.len() - todos_active.len() > 0;

        let handle_keydown = ctx.link().batch_callback(|event: KeyboardEvent| {
            if event.key() == "Enter" {
                Some(TodoAppMsg::Add)
            } else {
                None
            }
        });

        let handle_input = ctx.link().batch_callback(|e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                Some(TodoAppMsg::EditNewTodo(input.value()))
            } else {
                None
            }
        });

        let handle_onchange = ctx.link().batch_callback(|e: Event| {
            let target = e.target();
            let checkbox = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            checkbox.map(|c| TodoAppMsg::MarkAllComplete(c.checked()))
        });

        html! {
            <>
                <section class="todoapp">
                    <header class="header">
                        <h1>{ "todos" }</h1>
                        <input 
                            autofocus={true} 
                            value={self.new_todo.clone()} 
                            oninput={handle_input} 
                            onkeydown={handle_keydown} 
                            autocomplete="off" placeholder="What needs to be done?" 
                            class="new-todo" />
                    </header>

                    <section class="main">
                        <input
                            id="toggle-all" 
                            class="toggle-all" 
                            type="checkbox" 
                            onchange={handle_onchange}
                            checked={todos_active.len() == 0} />
                        <label for="toggle-all">{"Mark all as complete"}</label>
                        <ul class="todo-list">
                            {
                                todos
                                    .into_iter()
                                    .map(|todo| html!{ <TodoItem 
                                        todo={todo.clone()} 
                                        editing={false} 
                                        on_destroy={ctx.link().callback(|id: usize| { TodoAppMsg::Remove(id) })} 
                                        on_toggle={ctx.link().callback(|id: usize| { TodoAppMsg::Complete(id) })} />
                                    })
                                    .collect::<Html>()
                            }
                        </ul>
                    </section>

                    <footer class="footer">
                        <span class="todo-count"><strong>{format!("{} left", todos_active.len())}</strong></span>
                        <ul class="filters">
                            <li><a href="">{"All"}</a></li>
                            <li><a href="">{"Active"}</a></li>
                            <li><a href="">{"Completed"}</a></li>
                        </ul>
                        if has_completed_todos {
                            <button class="clear-completed ">{"Clear Completed"}</button>
                        }
                    </footer>
                
                </section>

                <footer class="info">
                    <p>{"Double-click to edit a todo"}</p>
                    <p>{"Created by "}<a href="">{"negativefix"}</a></p>
                    <p>{"Part of "} <a href="">{"TodoMVC"}</a></p>
                </footer>
            </>
        }
    
    }
}

fn main() {
    yew::start_app::<TodoApp>();
}

