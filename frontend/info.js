import { getVersion } from '@tauri-apps/api/app';
import mpl2 from "./license/mpl-2.0.txt?raw"
import apache2 from "./license/apache-2.0.txt?raw"
import mit from "./license/mit.txt?raw"
import frontend_packages from "./license/frontend.json"
import backend_packages from "./license/backend.json"

async function init_info() {
    document.querySelector("#info-version").textContent = await getVersion();
    

    const license = document.querySelector("#info-license");
    license.textContent = mpl2;
    license.classList.add("hidden");

    const license_switch = document.querySelector("#info-license-switch");
    license_switch.classList.add("lni-chevron-down-circle");
    license_switch.addEventListener("click", () => {
        if (license.classList.contains("hidden")) {
            license.classList.remove("hidden");
            license_switch.classList.remove("lni-chevron-down-circle");
            license_switch.classList.add("lni-chevron-up-circle");
        }
        else {
            license.classList.add("hidden");
            license_switch.classList.add("lni-chevron-down-circle");
            license_switch.classList.remove("lni-chevron-up-circle");
        }
    });

    const deps = document.querySelector("#info-deps");
    deps.classList.add("hidden");

    const deps_switch = document.querySelector("#info-deps-switch");
    deps_switch.classList.add("lni-chevron-down-circle");
    deps_switch.addEventListener("click", () => {
        if (deps.classList.contains("hidden")) {
            deps.classList.remove("hidden");
            deps_switch.classList.remove("lni-chevron-down-circle");
            deps_switch.classList.add("lni-chevron-up-circle");
        }
        else {
            deps.classList.add("hidden");
            deps_switch.classList.add("lni-chevron-down-circle");
            deps_switch.classList.remove("lni-chevron-up-circle");
        }
    });

    const dependencies = document.querySelector("#info-dependencies");
    const packages = [];
    for (const pkg of frontend_packages) {
        packages.push({
            name: pkg.name,
            version: pkg.installedVersion,
            authors: [pkg.author],
            license: pkg.licenseType
        });
    }

    for (const pkg of backend_packages) {
        if (pkg.name == "note-rs") { continue; }

        packages.push({
            name: pkg.name,
            version: pkg.version,
            authors: pkg.authors.split("|"),
            license: pkg.license
        });
    }

    packages.sort((a, b) => a.name > b.name);
    for (const pkg of packages) {
        const tr = document.createElement("tr");
        dependencies.appendChild(tr);

        let td = document.createElement("td");
        tr.appendChild(td);
        td.textContent = pkg.name;

        td = document.createElement("td");
        tr.appendChild(td);
        td.textContent = pkg.version;

        td = document.createElement("td");
        tr.appendChild(td);
        for (const author of pkg.authors) {
            const div = document.createElement("div");
            td.appendChild(div);
            div.textContent = author;
        }

        td = document.createElement("td");
        tr.appendChild(td);
        td.textContent = pkg.license;
    }

    const oss = document.querySelector("#info-oss-licenses");
    oss.classList.add("hidden");

    const oss_switch = document.querySelector("#info-oss-licenses-switch");
    oss_switch.classList.add("lni-chevron-down-circle");
    oss_switch.addEventListener("click", () => {
        if (oss.classList.contains("hidden")) {
            oss.classList.remove("hidden");
            oss_switch.classList.remove("lni-chevron-down-circle");
            oss_switch.classList.add("lni-chevron-up-circle");
        }
        else {
            oss.classList.add("hidden");
            oss_switch.classList.add("lni-chevron-down-circle");
            oss_switch.classList.remove("lni-chevron-up-circle");
        }
    });

    document.querySelector("#info-license-apache2").textContent = apache2;
    document.querySelector("#info-license-mit").textContent = mit;
}

export { init_info }
