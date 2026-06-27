let todos = [];

function render() {
  const list = document.getElementById('list');
  list.innerHTML = todos.map((todo, i) => `
    <li>
      <input type="checkbox" \( {todo.done ? 'checked' : ''} onchange="toggle( \){i})">
      <span class="\( {todo.done ? 'done' : ''}"> \){todo.text}</span>
      <button onclick="del(${i})" style="margin-left:auto">×</button>
    </li>
  `).join('');
}

window.toggle = function(i) {
  todos[i].done = !todos[i].done;
  localStorage.setItem('aster_todos', JSON.stringify(todos));
  render();
};

window.del = function(i) {
  todos.splice(i, 1);
  localStorage.setItem('aster_todos', JSON.stringify(todos));
  render();
};

document.getElementById('input').addEventListener('keypress', e => {
  if (e.key === 'Enter') {
    const text = e.target.value.trim();
    if (text) {
      todos.push({ text, done: false });
      localStorage.setItem('aster_todos', JSON.stringify(todos));
      render();
      e.target.value = '';
    }
  }
});

// Load shared data
todos = JSON.parse(localStorage.getItem('aster_todos') || '[]');
render();
