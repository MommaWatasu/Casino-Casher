// function to send dispatch when the button is clicked
function reserve() {
  // create XMLHttpRequest object
  var xhr = new XMLHttpRequest();

  // specify the URL and method of the request
  xhr.open("GET", "https://script.google.com/macros/s/AKfycbyBm3cfvtX_bJdskkwGVy0wMaA5yfXs-rdP_lgUQIk/dev");

  xhr.setRequestHeader("Access-Control-Allow-Origin", "https://script.google.com")

  // define function to process responce when the request is completed
  xhr.onload = function() {
    // when the request success
    if (xhr.status === 201) {
      // inform to the user
      alert("予約が完了しました");
    }
    // otherwise
    else {
      // show error message
      alert("予約に失敗しました: " + xhr.statusText + "\nHRの受付の者に教えていただけると幸いです");
    }
  };

  // create data to send
  var data = JSON.stringify({});

  // send the request
  xhr.send();
}

window.addEventListener("load", () => {
})