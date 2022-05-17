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
        function(e) {
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
            function(e) {
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
        function(e) {
            invoke({ cmd: 'queryConnectionProfiles', callback: 'loadQueriedConnectionProfilesSelect', query: e.target.value });
        }
    );
    document.getElementById("inputConnectionSettings").addEventListener("click", function(e) {});
    inputProfileSelect.addEventListener(
        "click",
        function(e) {
            if (inputProfileSelect.classList.contains("arrow-open")) {
                inputProfileSelect.classList.replace("arrow-open", "arrow-closed");
                inputProfileSelectOptions.classList.replace("options-open", "options-closed");
            } else {
                inputProfileSelect.classList.replace("arrow-closed", "arrow-open");
                inputProfileSelectOptions.classList.replace("options-closed", "options-open");
            }
        }
    );
    document.getElementById("tabBar").appendChild(generateTabButtonHTML(0, "Home", false))
}

function setTime() {
    let time;
    window.setInterval(function() {
        time = new Date;
        document.getElementById("time").innerHTML = "<h2>" + time.toLocaleTimeString("fr-ch") + "</h2>"
    }, 250);
}

function setHostname(hostname) {
    document.getElementById("hostname").innerHTML = "<h2>" + hostname + "</h2>";
}

function connect() {
    let hiddenProfileInput = document.getElementById("inputConnectionProfile");
    if (hiddenProfileInput.value != "") {
        invoke({
            cmd: 'connect',
            profile: JSON.parse(hiddenProfileInput.value)
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

    name.innerText = profile.name + " ";
    ip_fqdn.innerText = profile.connection_settings.ip_fqdn + " ";
    protocol.innerText = profile.connection_settings.protocol.name +
        " " + profile.connection_settings.protocol.port +
        " (" + profile.connection_settings.protocol.port_protocol + ")";

    name.classList.add("profileName");
    ip_fqdn.classList.add("profileIpFqdn");
    protocol.classList.add("profileProtocol");

    div.appendChild(name);
    div.appendChild(ip_fqdn);
    div.appendChild(protocol);

    div.addEventListener(
        "click",
        function(e) {
            invoke({ cmd: 'loadConnectionProfile', callback: 'loadSelectedConnectionProfileMenu', id: profile.id });
        }
    );

    return div;
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

function loadSelectedConnectionProfileMenu(profile) {
    document.getElementById("inputIpFqdn").value =
        profile.connection_settings.protocol.name +
        " " + profile.connection_settings.protocol.port +
        " (" + profile.connection_settings.protocol.port_protocol + ")";

    document.getElementById("inputConnectionProfile").value = JSON.stringify(profile);
}

function loadSelectedConnectionProfileSettings(profile) {
    document.getElementById("connectionProfilesSelect").value = profile.id;
    document.getElementById("connectionProfileID").value = profile.id;
    document.getElementById("connectionProfileName").value = profile.name;
    document.getElementById("connectionProfileConnectionSettings").value = JSON.stringify(profile.connection_settings);
    let networkProfiles = [...document.getElementById("connectionProfileNetworkProfilesSelect").options];
    networkProfiles.map(option => option.removeAttribute('selected'));
    networkProfiles.filter((profileOption) => {
        return profile.network_profiles.some((profile) => {
            return profile.uuid == JSON.parse(profileOption.value).uuid;
        });
    }).map(option => option.setAttribute('selected', 'selected'));
}

function saveSelectedConnectionProfile(profile) {
    profile.name = document.getElementById("connectionProfileName").value;
    profile.connection_settings = JSON.parse(document.getElementById("connectionProfileConnectionSettings").value);
    profile.network_profiles = [];
    let networkProfiles = document.getElementById("connectionProfileNetworkProfilesSelect").selectedOptions;
    for (let networkProfile of networkProfiles) {
        if (networkProfile.selected) {
            profile.network_profiles.push(JSON.parse(networkProfile.value));
        }
    }

    invoke({ cmd: 'saveConnectionProfile', profile: profile });
    invoke({ cmd: 'queryConnectionProfiles', callback: 'loadQueriedConnectionProfilesSettings', query: '' });
    invoke({ cmd: 'queryConnectionProfiles', callback: 'loadQueriedConnectionProfilesSelect', query: '' });
}

function deleteSelectedConnectionProfile(profile) {
    invoke({ cmd: 'deleteConnectionProfile', profile: profile });
    invoke({ cmd: 'queryConnectionProfiles', callback: 'loadQueriedConnectionProfilesSettings', query: '' });
    invoke({ cmd: 'queryConnectionProfiles', callback: 'loadQueriedConnectionProfilesSelect', query: '' });
}

function loadNetworkProfiles(profiles) {
    let networkProfilesSelectTag = document.getElementById("networkProfilesSelect");
    let connectionProfileNetworkProfilesSelectTag = document.getElementById("connectionProfileNetworkProfilesSelect");
    networkProfilesSelectTag.innerHTML = "";
    connectionProfileNetworkProfilesSelectTag.innerHTML = "";
    profiles.forEach(profile => {
        let optionTag = document.createElement("option");
        optionTag.value = JSON.stringify(profile);
        optionTag.innerText = profile.name;
        networkProfilesSelectTag.appendChild(optionTag);
        connectionProfileNetworkProfilesSelectTag.appendChild(optionTag.cloneNode(true));
    });
}

function loadSelectedNetworkProfile(profile) {
    invoke({ cmd: 'getNetworkInterfaces' });
    let fieldsets = document.getElementsByClassName("typeSpecific");
    for (let fieldset of fieldsets) {
        fieldset.classList.remove("currentType");
    }
    document.getElementsByName("networkProfileSpecific" + profile.profile_type)[0].classList.add("currentType")
    document.getElementById("networkProfileName").value = profile.name;
    document.getElementById("networkProfileDeviceSelect").value = JSON.stringify(profile.interface);
}

function saveSelectedNetworkProfile(profile) {
    profile.name = document.getElementById("networkProfileName").value;
    profile.interface = JSON.parse(document.getElementById("networkProfileDeviceSelect").value);
    invoke({ cmd: 'saveNetworkProfile', profile: profile });
    invoke({ cmd: 'getNetworkProfiles' });
}

function deleteSelectedNetworkProfile(profile) {
    invoke({ cmd: 'deleteNetworkProfile', profile: profile });
    invoke({ cmd: 'getNetworkProfiles' });
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

function addField(type, fieldsetID) {
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
                function() {
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
        optionTag.value = JSON.stringify(interface);
        optionTag.innerText = interface.interface_type + ": " + interface.name + " (" + interface.mac_addr + ")";
        selectTag.appendChild(optionTag);
    });
}

function toggleFieldset(sender, fieldsetID) {
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

function generateTabButtonHTML(tabId, tabName, closable) {
    let tabButton = document.createElement("div");
    let innerSpan = document.createElement("span");
    tabButton.classList.add("tabButton");
    tabButton.classList.add("currentTabButton");
    innerSpan.innerHTML = tabName;
    tabButton.setAttribute("sessionId", tabId);
    tabButton.addEventListener(
        "click",
        function(e) {
            showTab(tabId);
        }
    );
    if (closable) {
        let closeButton = document.createElement("button");
        closeButton.addEventListener(
            "click",
            function(e) {
                let tabContainer = document.getElementById("tabContainer");
                let tab = document.querySelector("div.tab[sessionId='" + tabId + "']");
                if (tab.classList.contains("currentTab")) {
                    if (tab.nextElementSibling != null) {
                        showTab(tab.nextElementSibling.attributes.sessionid.value);
                    } else {
                        showTab(tab.previousElementSibling.attributes.sessionid.value);
                    }
                }

                tabContainer.removeChild(tab);
                tabBar.removeChild(tabButton);
                invoke({ cmd: 'disconnect', id: tabId });
            }
        );
        closeButton.innerHTML = "x";
        innerSpan.appendChild(closeButton);
    }
    tabButton.appendChild(innerSpan);
    return tabButton
}

function openSessionTab(id, name, rfb_port) {
    let tabContainer = document.getElementById("tabContainer");
    let tabBar = document.getElementById("tabBar");
    tabBar.appendChild(generateTabButtonHTML(id, name, true));
    let tab = document.createElement("div");
    tab.setAttribute("sessionId", id);
    tab.classList.add("tab");
    //Generate noVnc page and connect to specified port
    tabContainer.appendChild(tab);
    showTab(id);
}

function showTab(tabId) {
    let tab = document.querySelector("div.tab[sessionId='" + tabId + "']");
    if (tab != null) {
        let tabButton = document.querySelector("div.tabButton[sessionId='" + tabId + "']");
        document.querySelector("div.currentTabButton").classList.remove("currentTabButton");
        document.querySelector("div.currentTab").classList.remove("currentTab");
        tab.classList.add("currentTab");
        tabButton.classList.add("currentTabButton");
    }
}

function showErrorPopup(message) {

}

function validateCIDR(e) {
    //console.log(e);
    return true;
}

function validateGateway(e) {
    //console.log(e);
    return true;
}