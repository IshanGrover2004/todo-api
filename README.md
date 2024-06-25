# Task Management API

A simplified task management web API built with Rust and Axum framework. This application allows users to create, update, delete, and list tasks, supporting multiple users with individual task sets.

## Requirements

- Rust (latest stable version)
- Cargo (comes with Rust)

## Installation

1. Clone the repository:

```bash
$ git clone https://github.com/IshanGrover2004/todo-api.git
$ cd todo-api
```

2. Build the project:

```bash
$ cargo build --release
```

## Usage

Run the server:

```bash
$ cargo run --release
```

Note: The server will start on `http://localhost:3000`

## Testing

Run the test suite:

```bash
$ cargo test
```

## API Endpoints

- `POST /users` -:

  - Create a new user
  - **Response**:

  ```json
  {user_id}
  ```

  - **Command**:

  ```bash
  $ curl -X POST http://localhost:3000/users
  ```

- `POST /users/:user_id/tasks` -:

  - Create a new task for the specified user
  - **Request Body** -:

  ```json
  {
    "title": "Task Title",
    "description": "Task Description",
    "due_date": "2023-05-01T00:00:00Z",
    "status": "Todo"
  }
  ```

  - **Command** example -:

  ```bash
    $ curl -X POST http://localhost:3000/users/{user_id}/tasks \
              -H "Content-Type: application/json" \
              -d '{"title": "Task Title", "description": "Task Description", "due_date": "2023-05-01T00:00:00Z", "status": "Todo"}'`
  ```

  - **Response** -: JSON of current stored TODO with task id

- `GET /users/:user_id/tasks` -:

  - Retrieve all tasks for the specified user
  - **Response** -:

  ```json
  [
    {
      "id": "1",
      "title": "Task 1",
      "description": "Description 1",
      "due_date": "2023-05-01T00:00:00Z",
      "status": "Todo"
    },
    {
      "id": "2",
      "title": "Task 2",
      "description": "Description 2",
      "due_date": "2023-05-02T00:00:00Z",
      "status": "InProgress"
    }
  ]
  ```

  - **Command** -:

  ```bash
  $ curl http://localhost:3000/users/{user_id}/tasks
  ```

- `GET /users/:user_id/tasks/:task_id` -:

  - Retrieve a specific task for the specified user
  - **Response** -:

  ```json
  {
    "id": "task_id",
    "title": "Task Title",
    "description": "Task Description",
    "due_date": "2023-05-01T00:00:00Z",
    "status": "Todo"
  }
  ```

  - **Command** -:

  ```bash
  $ curl http://localhost:3000/users/{user_id}/tasks/{task_id}
  ```

- `PUT /users/:user_id/tasks/:task_id` -:

  - Update a specific task for the specified user
  - **Request Body** -:

  ```json
  {
    "title": "Updated Task Title",
    "description": "Updated Task Description",
    "due_date": "2023-05-02T00:00:00Z",
    "status": "InProgress"
  }
  ```

  - **Response** -:

  ```json
  {
    "id": "task_id",
    "title": "Updated Task Title",
    "description": "Updated Task Description",
    "due_date": "2023-05-02T00:00:00Z",
    "status": "InProgress"
  }
  ```

  - **Command** example -:

  ```bash
    $ curl -X PUT http://localhost:3000/users/{user_id}/tasks/{task_id} \
        -H "Content-Type: application/json" \
        -d '{"title": "Updated Task", "description": "This task has been updated", "due_date": "2023-05-02T00:00:00Z", "status": "InProgress"}'`
  ```

  - **Response** -: JSON of current stored TODO with task id

- `DELETE /users/:user_id/tasks/:task_id` -:

  - Delete a specific task for the specified user
  - **Response** -: Status: 204 No Content
  - **Command** example -:

  ```bash
  $ curl -X DELETE http://localhost:3000/users/{user_id}/tasks/{task_id}
  ```
