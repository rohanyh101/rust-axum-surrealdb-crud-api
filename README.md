# RUST AXUM SURREALDB CRUD API

This project is a Rust-based API built using the Axum framework for handling CRUD operations on a SurrealDB instance running in a Docker container. 
SurrealDB provides a simple in-memory database solution, allowing users to store and retrieve data via HTTP requests. 
The API supports creating, reading, updating, and deleting todo items stored in the SurrealDB instance. 

## RUN INSTRUCTIONS,
1. Run SurrealDB in a Docker container using the following command:
```bash
docker run --rm --name surrealdb -p 8000:8000 surrealdb/surrealdb:latest start --user root --pass root --auth memory
```
2. Navigate to the project directory and execute `cargo run`.
3. The project will start running, and the API will be accessible at `localhost:3000`.

## ENDPOINTS TO CHECK,
- HEALTH CHECK (health_check_handler):
```powershell
curl --location --request GET 'localhost:3000/api/healthcheck' `
>> --header 'Content-Type: application/json' `
```

- CREATE TODO (create_todo_command):
```powershell
curl --location --request POST 'localhost:3000/api/todo' `
>> --header 'Content-Type: application/json' `
>> --data '{ "title": "new todo", "content": "my content", "is_completed": false }
```

- GET ALL TODOS (get_all_todo_query):
```powershell
curl --location --request GET 'localhost:3000/api/todo' `
>> --header 'Content-Type: application/json' `
```

- GET TODO BY ID (get_todo_by_id_query):
```powershell
curl --location --request GET 'localhost:3000/api/todo/tmwfqphx4jf9hfwdihpy' `
>> --header 'Content-Type: application/json' `
```

- UPDATE TODO (update_todo_command):
```powershell
curl --location --request PUT 'localhost:3000/api/todo/tmwfqphx4jf9hfwdihpy' `
>> --header 'Content-Type: application/json' `
>> --data '{ "title": "new todo", "content": "my content", "is_completed": false }
```

- DELETE TODO (delete_todo_command):
```powershell
curl --location --request DELETE 'localhost:3000/api/todo/tmwfqphx4jf9hfwdihpy' `
>> --header 'Content-Type: application/json' `
```
