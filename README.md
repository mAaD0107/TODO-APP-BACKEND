# TODO WEB SERVER


La aplicacion esta contruida en Rust Utiliza # Framework Actix
Bajo las siguientes dependencias:
[dependencies]

# Framework Web para Rust: Actix
# actix-web: Permite contruir servidores web y aplicaciones  

actix-web = "2.0.0"

# actix-rt: Permite crear tiempos de ejecucion asícronos a un subproceso 
 microservicio 
actix-rt = "1.0.0"  

# Framework para serializar y deserializar estructuras de datos  
serde = {version = "1.0.104", features = ["derive"]}

# Lee la configuracion desde el archivo .env para introducirlo en el entorno 
dotenv = "0.15.0"

Lee la configuracion desde diferentes fuentes y las carga en un tipo que esta definido en "config.rs"
Contiene el codigo de configuracion para las variables de entorno 
config = "0.10.1"

# Tokio es un controlador de eventos I/O para aplicaciones asincronas  tokio-pg-mapper: facilita el mapeo de estructuras de tablas de postgres  
tokio-pg-mapper = "0.1.4"
tokio-pg-mapper-derive = "0.1.4"
tokio-postgres = "0.5.1"

# deadpool-postgres: es un pool asincrono simple para postgres
deadpool-postgres = "0.5.0"

# Permite hacer get post al cliente local 
#rocket_cors = "0.5.1"
actix-cors = "0.5.3"
warp = "0.3.1"

Comandos curl para conectarse con la API

leer todos: curl http://localhost:8080/todos -s | jq .

leer todo por id_item: curl http://localhost:8080/todos/@id/items -s | jq .

insertar: curl -H "Content-Type: application/json" -X POST http://localhost:8080/todos/ -d "{"title":"Test Value"}"

actualizar chek: curl -X PUT http://localhost:8080/todos/2/items/3 -s | jq .

Requirements:

