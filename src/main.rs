mod todo;

use actix_web::{get, post, put, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
use todo::{Todo, TodoRequest};


struct AppState {
    todos: Mutex<Vec<Todo>>,
}

#[get("/todos")]
async fn get_todos(data: web::Data<AppState>) -> impl Responder {
    let todos = data.todos.lock().unwrap();
    let todos_json = serde_json::to_string(&*todos).unwrap();
    HttpResponse::Ok().json(todos_json)
}

#[post("/todos")]
async fn create_todo(data: web::Data<AppState>, new_todo: web::Json<TodoRequest>) -> impl Responder {
    let mut todos = data.todos.lock().unwrap();
    let id = todos.len() as i32 + 1;
    let todo = Todo {
        id,
        title: new_todo.title.clone(),
        completed: new_todo.completed,
    };
    todos.push(todo.clone());
    HttpResponse::Created().json(todo)
}   

#[put("/todos/{id}")]
async fn update_todo(
    data: web::Data<AppState>,
    id: web::Path<i32>,
    updated_todo: web::Json<TodoRequest>,
) -> impl Responder {
    let mut todos = data.todos.lock().unwrap();
    if let Some(todo) = todos.iter_mut().find(|t| t.id == *id) {
        todo.title = updated_todo.title.clone();
        todo.completed = updated_todo.completed;
        HttpResponse::Ok().json(todo.clone())
    } else {
        HttpResponse::NotFound().finish()
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        todos: Mutex::new(vec![
            Todo {
                id: 1,
                title: "Finish Actix tutorial".to_string(),
                completed: false,
            },
            Todo {
                id: 2,
                title: "Submit code review".to_string(),
                completed: false,
            },
            Todo {
                id: 3,
                title: "Walk the dog".to_string(),
                completed: true,
            },
        ]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(get_todos)
            .service(create_todo)
            .service(update_todo)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
