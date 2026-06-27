let todos = [];

function render() {
  const list = document.getElementById('list');
  list.innerHTML = '';

  todos.forEach((todo, i) => {
    const li = document.createElement('li');

    li.innerHTML = `
      <input type="checkbox" ${todo.done ? 'checked' : ''}>
      <span class="\( {todo.done ? 'done' : ''}"> \){todo.text}</span>
      <button>×</button>
    `;

    // Dynamic event listeners
    li.querySelector('input').addEventListener('change', () => toggleTodo(i));
    li.querySelector('button').addEventListener('click', () => deleteTodo(i));

    list.appendChild(li);
  });
}

function toggleTodo(i) {
  todos[i].done = !todos[i].done;
  saveTodos();
}

function deleteTodo(i) {
  todos.splice(i, 1);
  saveTodos();
}

function saveTodos() {
  chrome.storage.local.set({ todos: todos });
}

document.getElementById('input').addEventListener('keypress', e => {
  if (e.key === 'Enter') {
    const text = e.target.value.trim();
    if (text) {
      todos.push({ text, done: false });
      saveTodos();
      render();
      e.target.value = '';
    }
  }
});

// Load todos
chrome.storage.local.get('todos', (data) => {
  todos = data.todos || [];
  render();
});
