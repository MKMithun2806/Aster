let todos = [];

function render() {
  const list = document.getElementById('list');
  list.innerHTML = ''; // Clear first

  todos.forEach((todo, i) => {
    const li = document.createElement('li');
    li.innerHTML = `
      <input type="checkbox" ${todo.done ? 'checked' : ''}>
      <span class="\( {todo.done ? 'done' : ''}"> \){todo.text}</span>
      <button>×</button>
    `;

    li.querySelector('input').addEventListener('change', () => toggle(i));
    li.querySelector('button').addEventListener('click', () => del(i));

    list.appendChild(li);
  });
}

function toggle(i) {
  todos[i].done = !todos[i].done;
  save();
}

function del(i) {
  todos.splice(i, 1);
  save();
}

function save() {
  chrome.storage.local.set({ todos: todos });
}

document.getElementById('input').addEventListener('keypress', e => {
  if (e.key === 'Enter') {
    const text = e.target.value.trim();
    if (text) {
      todos.push({ text, done: false });
      save();
      render();
      e.target.value = '';
    }
  }
});

// Load
chrome.storage.local.get('todos', (data) => {
  todos = data.todos || [];
  render();
});
