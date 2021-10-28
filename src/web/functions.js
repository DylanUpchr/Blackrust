function invoke(arg) {
	window.webkit.messageHandlers.external.postMessage(JSON.stringify(arg));
}

function init() {
	setTime();
	invoke({cmd: 'init'});
	let inputIpFqdn = document.getElementById("inputIpFqdn");
	let inputProfile = document.getElementById("inputProfile");
	inputIpFqdn.addEventListener("input", 
		function(e){
			invoke({cmd: 'queryProfiles', query: e.target.value});
		});
	document.getElementById("inputConnect").addEventListener("click", function(e){});
	document.getElementById("inputConnectionSettings").addEventListener("click", function(e){});
	inputProfile.addEventListener(
		"change", 
		function(e){invoke({cmd: "loadProfile", profile: e.target.value})
	});
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

function connect(){
	let ip_fqdn = document.getElementById("inputIpFqdn").value;
	let protocol = document.getElementById("inputProtocol").value;
	let config = document.getElementById("inputConnectionSettingsValue").value;

	if(validateIpFqdn(ip_fqdn.value)){
		invoke({
			cmd: 'connect' ,
			ip_fqdn: ip_fqdn,
			protocol: protocol,
			config: config
		});
	}
}

function validateIpFqdn(value){
	let validIpAddressRegex = "^(([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])\.){3}([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])$";
	let validHostnameRegex = "^(([a-zA-Z0-9]|[a-zA-Z0-9][a-zA-Z0-9\-]+[a-zA-Z0-9])\.)+([A-Za-z0-9]|[A-Za-z0-9][A-Za-z0-9\-]+[A-Za-z0-9])$";
	let IpMatches = value.match(validIpAddressRegex);
	let HostnameMatches = value.match(validHostnameRegex);
	console.log(IpMatches);
	console.log(HostnameMatches);

	return (IpMatches != null || HostnameMatches != null)
}

function loadQueriedProfiles(profiles){
	invoke({cmd: 'debug', value: "Received result: " + JSON.stringify(profiles)});
}
