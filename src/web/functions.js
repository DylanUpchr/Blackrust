function invoke(arg) {
	window.webkit.messageHandlers.external.postMessage(JSON.stringify(arg));
}

function init() {
	setTime();
	invoke({cmd: 'init'});

	let inputProfileSelect = document.getElementById("inputProfileSelect");
	let inputProfileSelectOptions = document.getElementById("inputProfileSelectOptions");
	
	document.getElementById("inputIpFqdn").addEventListener("input", 
		function(e){
			invoke({cmd: 'queryProfiles', query: e.target.value});
		}
	);
	document.getElementById("inputConnect").addEventListener("click", function(e){});
	document.getElementById("inputConnectionSettings").addEventListener("click", function(e){});
	inputProfileSelect.addEventListener(
		"click", 
		function(e){
			if(inputProfileSelect.classList.contains("arrow-open")){
				inputProfileSelect.classList.replace("arrow-open", "arrow-closed");
				inputProfileSelectOptions.classList.replace("options-open", "options-closed");
			} else {
				inputProfileSelect.classList.replace("arrow-closed", "arrow-open");
				inputProfileSelectOptions.classList.replace("options-closed", "options-open");
			}
		}
	);
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

	return (IpMatches != null || HostnameMatches != null)
}

function loadQueriedProfiles(profiles){
	let inputProfileSelectOptions = document.getElementById("inputProfileSelectOptions");
	inputProfileSelectOptions.innerHTML = "";
	if (Object.keys(profiles).length == 0) {
		let div = document.createElement("div");
		let text = document.createElement("i");

		text.innerText = "No Profiles";
		
		div.appendChild(text);
		inputProfileSelectOptions.appendChild(text);
	} else {
		for (let key in profiles) {
			if (Object.hasOwnProperty.call(profiles, key)) {
				let profile = profiles[key];
				inputProfileSelectOptions.appendChild(genProfileSelectItemHTML(profile));
			}
		}
	}
}

function genProfileSelectItemHTML(profile){
	let div = document.createElement("div");
	let name = document.createElement("span");
	let ip_fqdn = document.createElement("span");
	let protocol = document.createElement("span");

	name.innerText = profile.name;
	ip_fqdn.innerText = profile.ip_fqdn;
	protocol.innerText = profile.protocol.name + " " + profile.protocol.port + " (" + profile.protocol.port_protocol + ")";

	name.classList.add("profileName");
	ip_fqdn.classList.add("profileIpFqdn");
	protocol.classList.add("profileProtocol");

	div.appendChild(name);
	div.appendChild(ip_fqdn);
	div.appendChild(protocol);

	div.addEventListener(
		"click", 
		function(e){
			invoke({cmd: 'loadProfile', id: profile.id});
		}
	);

	return div;
}

function loadSelectedProfile(profile){
	invoke({cmd: 'debug', value: JSON.stringify(profile)});
}
