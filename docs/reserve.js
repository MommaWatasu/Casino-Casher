const backend_url = "http://localhost:8000"

var times = new Array("9:30~10:00", "10:00~10:30", "10:30~11:00", "11:00~11:30", "11:30~12:00", "12:00~12:30", "12:30~13:00", "13:00~13:30", "13:30~14:00", "14:00~14:30", "14:30~15:00", "15:00~15:30", "15:30~16:00", "16:00~16:30")
let ip = "";

function reserve() {
  let time = document.getElementById("time-select").value;
  const xhr = new XMLHttpRequest();
  xhr.open("GET", backend_url+"/register?time="+String(time)+"&ip="+String(ip));
  xhr.send();
  xhr.responseType = "json";
  xhr.onload = () => {
    if (xhr.readyState == 4 && xhr.status == 200) {
      const data = xhr.response;
      if (data.status) {
        alert("予約できました");
        window.location.reload();
      } else if (data.err == 2) {
        alert("予約に失敗しました。無効な時間です。");
      } else {
        alert("予約に失敗しました。");
      }
    } else {
      alert(`予約に失敗しました: ${xhr.status}`);
    }
  };
}

window.addEventListener("load", async function() {
  ip = await fetch('https://ipinfo.io?callback')
    .then(res => res.json())
    .then(json => console.log(json.ip));
  const xhr = new XMLHttpRequest();
  xhr.open("GET", backend_url+"/get_time?ip="+String(ip));
  xhr.send();
  xhr.responseType = "json";
  xhr.onload = () => {
    if (xhr.readyState == 4 && xhr.status == 200) {
      const data = xhr.response;
      if (data.time != -1) {
        document.getElementById("reserve").innerHTML = `<p id="result">${times[data.time-1]}</p>`;
      }
    } else {}
  };
});