import React  from "react";
import { TodoCounter } from "./TodoCounter";
import { TodoSearch } from "./TodoSearch.js";
import { TodoList } from "./TodoList.js";
import { TodoItem } from "./TodoItem.js";
import { CreateTodoButton } from "./CreateTodoButton.js";
// import './App.css';

const todos = [
  {text: "Ir al supermercado", completed: false},
  {text: "Tomar agua", completed: false},
  {text: "Hacer ejercicio", completed: false},
];

function App() {
  return (
    <React.Fragment>
      <TodoCounter />
      <TodoSearch />
      <TodoList>
        {todos.map(todo => (
          <TodoItem key={todo.text} text={todo.text}/>
          ))}
      </TodoList>
      <CreateTodoButton />
    </React.Fragment>
  );
};

export default App;
