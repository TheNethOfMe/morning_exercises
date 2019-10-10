import { action, thunk } from "easy-peasy";
import uuid from "uuid";

export default {
  todos: [],
  // Thunks
  fetchTodos: thunk(async actions => {
    const res = await fetch(
      "https://jsonplaceholder.typicode.com/todos?_limit=5"
    );
    const todos = await res.json();
    actions.setTodos(todos);
  }),
  // Actions
  setTodos: action((state, todos) => {
    state.todos = todos;
  }),
  toggle: action((state, id) => {
    state.todos.map(todo => {
      if (todo.id === id) {
        todo.completed = !todo.completed;
      }
      return todo;
    });
  }),
  remove: action((state, id) => {
    state.todos = state.todos.filter(todo => todo.id !== id);
  }),
  add: action((state, text) => {
    const newTodo = {
      id: uuid.v4(),
      title: text,
      completed: false
    };
    state.todos = [...state.todos, newTodo];
  })
};
