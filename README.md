================================================ 
          TODO WEB SERVER
================================================

Comandos curl para conectarse con la API

leer todos: curl http://localhost:8080/todos -s | jq .

leer todo por id_item: curl http://localhost:8080/todos/@id/items -s | jq .

insertar: curl -H "Content-Type: application/json" -X POST http://localhost:8080/todos/ -d "{"title":"Test Value"}"

actualizar chek: curl -X PUT http://localhost:8080/todos/2/items/3 -s | jq .

Requirements:

La aplicacion esta contruida en Rust Utiliza # Framework Actix
