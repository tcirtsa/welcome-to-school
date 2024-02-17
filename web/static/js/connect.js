const insert_user=document.querySelector("#insert_user");
function makeCellEditable(cell1,cell2) {
  const originalText = cell2.innerText; // 保存原始文本
  cell2.innerHTML = ''; // 清空单元格内容
  const input = document.createElement('input'); // 创建输入框
  input.type = 'text';
  input.value = originalText;
  input.style.width = '100%';
  cell2.appendChild(input); // 单元格内添加输入框
  input.focus();

  // 输入框失去焦点时保存更改并恢复到文本模式
  input.addEventListener('blur', () => {
      const newText = input.value.trim();
      cell2.innerHTML = newText; // 更新单元格文本
      // 可在此处发送更新到服务器的请求
      fetch('http://localhost:8080/update_psd', {
        method: 'POST', // or 'PUT'
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({account:cell1.innerText,psd:originalText}),
      })
      .then((response) => response.json())
      .then((data) => {
        console.log('Success:', data);
      })
      .catch((error) => {
        console.error('Error:', error);
      });
      
  });

  // 输入框内按下回车键时同样保存更改
  input.addEventListener('keydown', (e) => {
      if (e.key === 'Enter') {
          input.blur(); // 触发blur事件
      }
  });
}
function fetchData() {
  // 清空表格并重新加载数据
  function loadData() {
      fetch('http://localhost:8080/get_all_accounts',{
        method: 'POST', // or 'PUT'
  headers: {
    'Content-Type': 'application/json',
  },
      })
          .then(response => response.json())
          .then(data => {
            const tableBody = document.getElementById('data-table').getElementsByTagName('tbody')[0];
            tableBody.innerHTML = ''; // 清空现有表格内容
    
            data.forEach(row => {
                const tr = document.createElement('tr');
                
                // 创建ID单元格
                const idCell = document.createElement('td');
                idCell.innerText = row.id;
                tr.appendChild(idCell);
                
                // 创建account单元格
                const accountCell = document.createElement('td');
                accountCell.innerText = row.account;
                tr.appendChild(accountCell);
                
                // 创建psd单元格
                const psdCell = document.createElement('td');
                psdCell.innerText = row.psd;
                psdCell.addEventListener('click', () => makeCellEditable(accountCell,psdCell)); // 点击变为编辑状态
                tr.appendChild(psdCell);

                // 创建points单元格
                const pointsCell = document.createElement('td');
                pointsCell.innerText = row.points;
                tr.appendChild(pointsCell);

                // 不可编辑的其他数据单元格...

                tableBody.appendChild(tr);
            });
        })
        .catch(err => console.error('Error fetching data:', err));
  }

  // 加载数据
  loadData();
}
insert_user.addEventListener("click",()=>{
  let url = 'http://localhost:8080/check_account';
let data = { account:'1220204156',psd:'123456' };
fetch(url, {
  method: 'POST', // or 'PUT'
  headers: {
    'Content-Type': 'application/json',
  },
  body: JSON.stringify(data),
})
.then((response) => response.json())
.then((data) => {
  console.log('Success:', data);
})
.catch((error) => {
  console.error('Error:', error);
});
})

// 页面刚加载完毕时自动获取数据
document.addEventListener('DOMContentLoaded', fetchData);

// 添加点击事件来手动刷新表格数据
document.getElementById('refreshButton').addEventListener('click', fetchData);
