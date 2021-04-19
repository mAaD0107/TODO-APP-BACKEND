

use crate::model::{TodoList, TodoItem};
use deadpool_postgres::Client; 
use tokio_pg_mapper::FromTokioPostgresRow; 
use std::io;

pub async fn get_todos(client: &Client) -> Result<Vec<TodoList>, io::Error>{
    let statment = client.prepare("select * from todo_list order by id desc").await.unwrap(); 

    let todos = client.query(&statment, &[])
        .await
        .expect("Error al obtener la lista Todo")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>();

    Ok(todos)
}


pub async fn get_items(client: &Client, list_id: i32) -> Result<Vec<TodoItem>, io::Error>{
    let statment = client.prepare("select * from todo_item where list_id = $1 order by id").await.unwrap(); 

    let items = client.query(&statment, &[&list_id])
        .await
        .expect("Error al obtener la lista Todo")
        .iter()
        .map(|row| TodoItem::from_row_ref(row).unwrap())
        .collect::<Vec<TodoItem>>();

    Ok(items)
}


pub async fn create_todo(client: &Client, title: String) -> Result<TodoList, io::Error>{
    let statment = client.prepare("insert into todo_list (title) values ($1) returning id, title").await.unwrap();

    client.query(&statment, &[&title])
    .await
    .expect("Error creando el Todo")
    .iter()
    .map(|row| TodoList::from_row_ref(row).unwrap())
    .collect::<Vec<TodoList>>()
    .pop()
    .ok_or(io::Error::new(io::ErrorKind::Other, "Error creando el Todo"))
}

pub async fn create_item(client: &Client, list_id: i32, title: String) -> Result<TodoList, io::Error>{
    let statment = client.prepare("insert into todo_ (list_id, title) values ($1, $2) returning id, title").await.unwrap();

    client.query(&statment, &[&title])
    .await
    .expect("Error creando el Todo")
    .iter()
    .map(|row| TodoList::from_row_ref(row).unwrap())
    .collect::<Vec<TodoList>>()
    .pop()
    .ok_or(io::Error::new(io::ErrorKind::Other, "Error creando el Todo"))
}


pub async fn check_item(client: &Client, list_id: i32, item_id: i32) -> Result<(), io::Error>{
    
    let statment = client.prepare("update todo_item set checked = 
    (CASE WHEN checked = false
     THEN true
     ELSE false
     END) 
    where list_id = $1 and id = $2").await.unwrap();

    let result = client.execute(&statment, &[&list_id,&item_id ])
    .await
    .expect("Error revisando el item");

    match result {
        ref updated if *updated == 1 => Ok(()),
        _ => Err(io::Error::new(io::ErrorKind::Other, "Fallo la revision del check"))
    }    
}



