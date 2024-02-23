let selectedRow = null; // 存储当前选中行的元素引用

// 初始化高德地图
var map = new AMap.Map('map', {
    resizeEnable: true,
    zoom: 10,
    center: [116.397428, 39.90923]
});

// 监听地图点击事件
AMap.event.addListener(map, 'click', function(e) {
    if (selectedRow) {
        selectedRow.querySelector('.longitude-input').value = e.lnglat.getLng();
        selectedRow.querySelector('.latitude-input').value = e.lnglat.getLat();
    }
});

// 在表格顶部添加一个新的数据行
document.getElementById('add-top').addEventListener('click', function() {
    const table = document.getElementById('data-table');
    const row = createDataRow();
    table.insertAdjacentElement('afterend', row);
});

// 提交数据按钮
document.getElementById('submit-data').addEventListener('click', function() {
    const table = document.getElementById('data-table');
    const rows = table.querySelectorAll('tr');
    const data = Array.from(rows).slice(1).map(row => {
        return {
            id: row.querySelector('.id-input').value,
            longitude: row.querySelector('.longitude-input').value,
            latitude: row.querySelector('.latitude-input').value
        };
    });

    // 使用AJAX发送数据到服务器
    fetch('http://localhost:8080/updata_map', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(data)
    }).then(response => {
        return response.json();
    }).then(result => {
        console.log('Data submitted successfully:', result);
    }).catch(error => {
        console.error('Error submitting data:', error);
    });
});

// 加载数据
function loadData() {
    fetch('http://localhost:8080/get_all_map',{
        method: 'POST', // or 'PUT'
    headers: {
        'Content-Type': 'application/json',
    },
    })
    .then(response => response.json())
    .then(data=>{
    const table = document.getElementById('data-table');
    data.forEach((row) => {
        const Row = createDataRow(row);
        table.appendChild(Row);
    });
    })
}

// 创建一个新的表格行
function createDataRow(data = { id: '', longitude: '', latitude: '' }) {
    const row = document.createElement('tr');

    row.innerHTML = `
        <td><input type="text" class="id-input" value="${data.id}"/></td>
        <td><input type="text" class="longitude-input" value="${data.longitude}"/></td>
        <td><input type="text" class="latitude-input" value="${data.latitude}"/></td>
        <td><button class="select-row">选择</button></td>
        <td>
            <button class="delete-row">删除</button>
            <button class="add-row-below">添加行</button>
        </td>
    `;

    // 选择行按钮
    const selectBtn = row.querySelector('.select-row');
    selectBtn.addEventListener('click', function() {
        if (selectedRow) {
            selectedRow.classList.remove('selected');
        }
        selectedRow = row;
        row.classList.add('selected');
    });

    // 删除行按钮
    const deleteBtn = row.querySelector('.delete-row');
    deleteBtn.addEventListener('click', function() {
        if (selectedRow == row) {
            selectedRow = null;
        }
        row.remove();
    });

    // 在当前行下方添加行按钮
    const addRowBelowBtn = row.querySelector('.add-row-below');
    addRowBelowBtn.addEventListener('click', function() {
        const newRow = createDataRow();
        row.parentNode.insertBefore(newRow, row.nextSibling);
    });

    return row;
}

// 调用 loadData 来初始化表格数据
loadData();