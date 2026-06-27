let todos = [];

function render() {
  const list = document.getElementById('list');
  list.innerHTML = todos.map((t, i) => `
    <li>
      <input type="checkbox" \( {t.done ? 'checked' : ''} onchange="toggle( \){i})">
      <span class="\( {t.done ? 'done' : ''}"> \){t.text}</span>
      <button onclick="del(${i})" style="margin-left:auto">×</button>
    </li>
  `).join('');
}

function toggle(i) {
  todos[i].done = !todos[i].done;
  save();
  render();
}

function del(i) {
  todos.splice(i, 1);
  save();
  render();
}

document.getElementById('input').addEventListener('keypress', e => {
  if (e.key === 'Enter' && e.target.value.trim()) {
    todos.push({text: e.target.value.trim(), done: false});
    save();
    render();
    e.target.value = '';
  }
});

function save() {
  localStorage.setItem('todos', JSON.stringify(todos));
}

// Load
todos = JSON.parse(localStorage.getItem('todos') || '[]');
render();
