function invoke(arg) {
	window.webkit.messageHandlers.external.postMessage(JSON.stringify(arg));
}

function init() {
	setTime();
	invoke({ cmd: 'init' });
	invoke({ cmd: 'getNetworkProfiles' });
	invoke({ cmd: 'queryConnectionProfiles', callback: 'loadQueriedConnectionProfilesSelect', query: '' });

	let inputProfileSelect = document.getElementById("inputProfileSelect");
	let inputProfileSelectOptions = document.getElementById("inputProfileSelectOptions");
	let settingsButton = document.getElementById("settingsButton");
	settingsButton.addEventListener(
		"click",
		function (e) {
			if (document.getElementsByClassName("currentContent")[0].id == "home") {
				showContent("settings");
				settingsButton.innerHTML = '<i class="fas fa-xmark"></i>';
			} else {
				showContent("home");
				settingsButton.innerHTML = '<i class="fas fa-cogs"></i>';
			}
		}
	);
	let subContentTabButtons = document.getElementsByClassName("subcontentTabButton");
	for (let button of subContentTabButtons) {
		button.addEventListener(
			"click",
			function (e) {
				showSubContent("settings", "settings" + button.attributes.name.value + "Subcontent");
				let content = document.getElementsByClassName("currentSubcontentTabButton");
				for (let elem of content) {
					elem.classList.remove("currentSubcontentTabButton");
				}
				button.classList.add("currentSubcontentTabButton");
			}
		);
	}

	document.getElementById("inputIpFqdn").addEventListener("input",
		function (e) {
			invoke({ cmd: 'queryConnectionProfiles', callback: 'loadQueriedConnectionProfiles', query: e.target.value });
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

function loadQueriedConnectionProfilesSelect(profiles) {
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
				inputProfileSelectOptions.appendChild(genConnectionProfileSelectItemHTML(profile));
			}
		}
	}
}

function genConnectionProfileSelectItemHTML(profile) {
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
			invoke({ cmd: 'loadConnectionProfile', callback: 'loadSelectedConnectionProfile', id: profile.id });
		}
	);

	return div;
}

function genConnectionProfileSettingsHTML(profile) {
}

function loadQueriedConnectionProfilesSettings(profiles) {
	let selectTag = document.getElementById("connectionProfilesSelect");
	selectTag.innerHTML = "";
	profiles = profiles['profile_vec'];
	profiles.forEach(profile => {
		let optionTag = document.createElement("option");
		optionTag.value = profile.id;
		optionTag.innerText = profile.name;
		selectTag.appendChild(optionTag);
	});
}
function loadSelectedConnectionProfile(profile) {
	document.getElementById("connectionProfilesSelect").value = profile.id;
	document.getElementById("connectionProfileID").value = profile.id;
}

function loadNetworkProfiles(profiles) {
	let selectTag = document.getElementById("networkProfilesSelect");
	selectTag.innerHTML = "";
	profiles.forEach(profile => {
		let optionTag = document.createElement("option");
		optionTag.value = profile.uuid;
		optionTag.innerText = profile.name;
		selectTag.appendChild(optionTag);
	});
}
function loadSelectedNetworkProfile(profile) {
	invoke({cmd: 'getNetworkInterfaces'});
	let fieldsets = document.getElementsByClassName("typeSpecific");
	for (let fieldset of fieldsets) {
		fieldset.classList.remove("currentType");
	}
	document.getElementsByName("networkProfileSpecific" + profile.profile_type)[0].classList.add("currentType")
	document.getElementById("networkProfileName").value = profile.name;
	document.getElementById("networkProfileDeviceSelect").value = profile.interface.name + profile.interface.mac_addr;
}

function showContent(contentName) {
	//Hide current content
	let content = document.getElementsByClassName("currentContent");
	for (let elem of content) {
		elem.classList.remove("currentContent");
	}

	//Show selected content
	document.getElementById(contentName).classList.add("currentContent");
}

function showSubContent(contentName, subContentTabName) {
	contentName = contentName + "Subcontent"
	let subcontentContainer = document.getElementById(contentName);
	for (let elem of subcontentContainer.children) {
		elem.classList.remove("currentSubcontentTab");
	}
	document.getElementById(subContentTabName).classList.add("currentSubcontentTab");
	switch (subContentTabName) {
		case "settingsNetworkSubcontent":
			invoke({ cmd: 'getNetworkProfiles' });
			break;
		case "settingsProfilesSubcontent":
			invoke({ cmd: 'queryConnectionProfiles', callback: 'loadQueriedConnectionProfilesSettings', query: '' });
			break;
		default:
			break;
	}
}

function addField(type, fieldsetID){
	let fieldset = document.getElementById(fieldsetID);
	switch (type) {
		case 'address':
			let br = document.createElement("br");
			let input = document.createElement("input");
			input.addEventListener("change", validateCIDR);
			input.required = true;
			let removeButton = document.createElement("button")
			removeButton.innerText = "Remove";
			removeButton.addEventListener(
				"click", 
				function(){
					fieldset.removeChild(input); 
					fieldset.removeChild(removeButton); 
					fieldset.removeChild(br)
				}
			);
			fieldset.insertBefore(br, fieldset.firstChild);
			fieldset.insertBefore(removeButton, fieldset.firstChild);
			fieldset.insertBefore(input, fieldset.firstChild);
			break;
		case 'gateway':
				
			break;
		case 'route':
					
			break;	
		default:
			break;
	}
}

function loadNetworkInterfaces(interfaces) {
	let selectTag = document.getElementById("networkProfileDeviceSelect");
	selectTag.innerHTML = "";
	interfaces.forEach(interface => {
		let optionTag = document.createElement("option");
		optionTag.value = interface.name + interface.mac_addr;
		optionTag.innerText = interface.interface_type + ": " + interface.name + " (" + interface.mac_addr + ")";
		selectTag.appendChild(optionTag);
	});
}

function toggleFieldset(sender, fieldsetID){
	let fieldset = document.getElementById(fieldsetID);
	if (fieldset.classList.contains("fieldset-closed")) {
		fieldset.classList.remove("fieldset-closed");
		fieldset.classList.add("fieldset-open");
		sender.innerText = "Hide";
	} else {
		fieldset.classList.remove("fieldset-open");
		fieldset.classList.add("fieldset-closed");
		sender.innerText = "Show";
	}
}

function validateCIDR(e){
	//console.log(e);
	return true;
}

function validateGateway(e){
	//console.log(e);
	return true;
}