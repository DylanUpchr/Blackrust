function invoke(arg) {
	window.webkit.messageHandlers.external.postMessage(JSON.stringify(arg));
}

function init() {
	setTime();
	invoke({cmd: 'init'});
}

function setTime() {
	let time;
	window.setInterval(function () {
		time = new Date;
		document.getElementById("time").innerHTML = "<h2>" + time.toLocaleTimeString("fr-ch") + "</h2>"
	}, 250);
}

function setHostname(hostname){
    document.getElementById("hostname").innerHTML = "<h2>" + hostname + "</h2>";
}

