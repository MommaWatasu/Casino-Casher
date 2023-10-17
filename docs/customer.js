// function to send dispatch when the button is clicked
function sendRequest() {
    // create XMLHttpRequest object
    var xhr = new XMLHttpRequest();
  
    // specify the URL and method of the request
    xhr.open("POST", "https://api.github.com/repos/MommaWatasu/Casino-Casher/dispatches");
  
    // set request headers
    xhr.setRequestHeader("Accept", "application/vnd.github+json");
    xhr.setRequestHeader("Authorization", "Bearer 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");
    xhr.setRequestHeader("X-GitHub-Api-Version", "2022-11-28");
  
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
    var data = JSON.stringify({
      event_type: "customer",
      client_payload: {
        number: "1"
      }
    });
  
    // send the request
    xhr.send(data);
  }
  
  window.addEventListener("load", () => {
      // get the button which has the `reserved` for id
      var button = document.getElementById("reserve");
      // add event listener for the button
      button.addEventListener("click", sendRequest);
  })