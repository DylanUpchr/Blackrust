function invoke(arg) {
	window.webkit.messageHandlers.external.postMessage(JSON.stringify(arg));
}

function init() {
	setTime();
	invoke({ cmd: 'init' });

	let inputProfileSelect = document.getElementById("inputProfileSelect");
	let inputProfileSelectOptions = document.getElementById("inputProfileSelectOptions");

	document.getElementById("settingsButton").addEventListener(
		"click",
		function (e) {
			if (document.getElementsByClassName("currentContent")[0].id == "home") {
				showContent("settings");
			} else {
				showContent("home");
			}
		}
	);

	document.getElementById("inputIpFqdn").addEventListener("input",
		function (e) {
			invoke({ cmd: 'queryProfiles', query: e.target.value });
		}
	);
	document.getElementById("inputConnect").addEventListener("click", function (e) { });
	document.getElementById("inputConnectionSettings").addEventListener("click", function (e) { });
	inputProfileSelect.addEventListener(
		"click",
		function (e) {
			if (inputProfileSelect.classList.contains("arrow-open")) {
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

function setHostname(hostname) {
	document.getElementById("hostname").innerHTML = "<h2>" + hostname + "</h2>";
}

function connect() {
	let ip_fqdn = document.getElementById("inputIpFqdn").value;
	let protocol = document.getElementById("inputProtocol").value;
	let config = document.getElementById("inputConnectionSettingsValue").value;

	if (validateIpFqdn(ip_fqdn.value)) {
		invoke({
			cmd: 'connect',
			ip_fqdn: ip_fqdn,
			protocol: protocol,
			config: config
		});
	}
}

function loadQueriedProfiles(profiles) {
	profiles = profiles['profile_vec']
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

function genProfileSelectItemHTML(profile) {
	let div = document.createElement("div");
	let name = document.createElement("span");
	let ip_fqdn = document.createElement("span");
	let protocol = document.createElement("span");

	name.innerText = profile.name;
	ip_fqdn.innerText = profile.connection_settings.ip_fqdn;
	protocol.innerText = profile.connection_settings.protocol.name + " " + profile.connection_settings.protocol.port + " (" + profile.connection_settings.protocol.port_protocol + ")";

	name.classList.add("profileName");
	ip_fqdn.classList.add("profileIpFqdn");
	protocol.classList.add("profileProtocol");

	div.appendChild(name);
	div.appendChild(ip_fqdn);
	div.appendChild(protocol);

	div.addEventListener(
		"click",
		function (e) {
			invoke({ cmd: 'loadProfile', id: profile.id });
		}
	);

	return div;
}

function loadSelectedProfile(profile) {
	invoke({ cmd: 'debug', value: JSON.stringify(profile) });
}

function showContent(contentName) {
	//Hide current content
	let content = document.getElementsByClassName("currentContent");
	for (let elem of content) {
		elem.classList.remove("currentContent");
	}

	//Show selected content
	let newContent = document.getElementById(contentName);
	newContent.classList.add("currentContent");
}

function showSubContent(contentName, subContentName) {

}