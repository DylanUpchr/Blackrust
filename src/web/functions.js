const DEFAULT_WIDGET = "time-widget"

function invoke(arg) {
	window.webkit.messageHandlers.external.postMessage(JSON.stringify(arg));
}

function init() {
	setTime();
	showWidget(DEFAULT_WIDGET);
	invoke({cmd: 'init'});
}

function setTime() {
	let time;
	window.setInterval(function () {
		time = new Date;
		document.getElementById("time-widget").innerHTML = "<h1>" + time.toLocaleTimeString("fr-ch") + "</h1>"
	}, 250);
}

function showWidget(widgetName) {
	for (let widget of document.getElementsByClassName("widget")) {
		widget.style.display = "none";
	}
	document.getElementById(widgetName).style.display = "flex";
}

function attemptLogin(){
	let form = document.getElementById("login-form");
	let username = form.elements["username-field"].value;
	let password = form.elements["password-field"].value;
	invoke({cmd : 'loginAttempt', username: username, password: password});
}