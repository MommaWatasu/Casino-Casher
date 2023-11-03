// const backend_url = "https://casher-yy7tpgmgqa-uw.a.run.app"
const backend_url = "http://localhost:8000"
var times = new Array("9:30~10:00", "10:00~10:30", "10:30~11:00", "11:00~11:30", "11:30~12:00", "12:00~12:30", "12:30~13:00", "13:00~13:30", "13:30~14:00", "14:00~14:30", "14:30~15:00", "15:00~15:30", "15:30~16:00", "16:00~16:30")
let ip = "";

function reserve() {
  let time = document.getElementById("time-select").value;
  const xhr = new XMLHttpRequest();
  xhr.open("GET", backend_url+"/reserve?time="+String(time)+"&ip="+String(ip));
  xhr.send();
  xhr.responseType = "json";
  xhr.onload = () => {
    if (xhr.readyState == 4 && xhr.status == 200) {
      const data = xhr.response;
      if (data.status == 0) {
        alert("予約できました");
        window.location.reload();
      } else if (data.status == 1) {
        alert("予約に失敗しました");
      } else if (data.status == 2) {
        alert("予約に失敗しました\n定員オーバーなので、他の時間を選択してください")
      } else if (data.status == 3) {
        alert("予約に失敗しました\n無効な時間です");
      } else {
        alert("予約に失敗しました\n1時間半後以降は無効です")
      }
    } else {
      alert(`予約に失敗しました: ${xhr.status}`);
    }
  };
}

function register() {
  const xhr = new XMLHttpRequest();
  xhr.open("POST", backend_url+"/register?ip="+String(ip));
  xhr.send();
  xhr.responseType = "text";
  xhr.onload = () => {
    if (xhr.readyState == 4 && xhr.status == 200) {
      alert("整理券を発行しました");
      window.location.reload();
    }
  }
}

function cancel_wait() {
  const xhr = new XMLHttpRequest();
  xhr.open("POST", backend_url+"/remove_wait?ip="+String(ip));
  xhr.send();
  xhr.responseType = "text";
  xhr.onload = () => {
    if (xhr.readyState == 4 && xhr.status == 200) {
      const data = xhr.response;
      if (data == "true") {
        alert("キャンセルしました");
        window.location.reload();
      } else {
        alert("キャンセルに失敗しました\n無効なIPアドレスです")
      }
    } else {
      alert(`キャンセルに失敗しました: ${xhr.status}`);
    }
  }
}

function cancel_reserve() {
  const xhr = new XMLHttpRequest();
  xhr.open("POST", backend_url+"/remove_reserve?ip="+String(ip));
  xhr.send();
  xhr.responseType = "text";
  xhr.onload = () => {
    if (xhr.readyState == 4 && xhr.status == 200) {
      const data = xhr.response;
      if (data == "true") {
        alert("キャンセルしました");
        window.location.reload();
      } else {
        alert("キャンセルに失敗しました\n無効なIPアドレスです")
      }
    } else {
      alert(`キャンセルに失敗しました: ${xhr.status}`);
    }
  }
}

window.addEventListener("load", async function() {
  ip = await fetch('https://ipinfo.io?callback')
    .then(res => res.json())
    .then(json => json.ip)
    .catch((err) => {
      if (!alert("IPアドレスの取得に失敗しました。広告ブロックやトラッキングを解除してください")) {
        window.location.reload();
      }
    });
  const xhr = new XMLHttpRequest();
  xhr.open("GET", backend_url+"/check_user?ip="+String(ip));
  xhr.send();
  xhr.responseType = "json";
  xhr.onload = () => {
    if (xhr.readyState == 4 && xhr.status == 200) {
      const data = xhr.response;
      if (data.user_type == 1) {
        document.getElementById("reserve").innerHTML = `<p id="result">${times[data.time-1]}</p><br><p>予約をキャンセルする場合は以下のボタンを押してください</p><div class="button" onclick="cancel_reserve()">キャンセル</div>`;
        document.getElementById("register").innerHTML = ""; 
      } else if (data.user_type == 2) {
        document.getElementById("reserve").innerHTML = "";
        document.getElementById("register").innerHTML = `<p id=result>${data.number}番目</p><br><p>整理券をキャンセルする場合は以下のボタンを押してください</p><div class="button" onclick="cancel_wait()">キャンセル</div>`;
      }
    }
  };
});
