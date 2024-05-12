const get_all_css_rules = () => {
    const result = new Map();
    const stylesheets = document.styleSheets;
    for(let i = 0; i < stylesheets.length; i++) {
        const stylesheet = stylesheets[i];
        const rules = stylesheet.cssRules;
        for (let j=0; j < rules.length; j++) {
            const rule = rules[j];
            result.set(rule.selectorText, rule);
        }
    }
    return result;
}

const get_rule = (name) => {
    const rules = get_all_css_rules();
    return rules.has(name) ? rules.get(name) : null;

}

function setColor(ruleName, color) {
    const rule = get_rule(ruleName);
    rule.style.background=color;
}


class SettingsPage
{
    #provider;
    #oninit;

    constructor(provider, oninit) {
        this.#provider = provider;
        this.#oninit = oninit;
        this.init();
    }

    async init() {
        const element = document.querySelector("#settings_items");

        const settings = await this.#provider.readAll();
        for(const setting of settings) {
            this.#addSetting(element, setting);
            this.#handleKnownSetting(setting.id, setting.value);
        }
    }

    #addSetting(element, setting) {
        switch (setting.data_type) {
            case "string":
                this.#addStringSetting(element, setting);
                break;
            case "color":
                this.#addColorSetting(element, setting);
                break;
            default:
                console.warn("setting: unknown data_type: ", setting);
                break;
        }
    }

    #addGenericSetting(element, setting, inputType) {
        const div = document.createElement("div");
        element.appendChild(div);

        const label = document.createElement("span");
        div.appendChild(label);
        label.textContent = setting.name;

        const input = document.createElement("input");
        div.appendChild(input);
        input.type = inputType;
        input.value = setting.value;

        input.addEventListener("change", () => {
            this.#write(setting.id, input.value);
        });
    }

    #addStringSetting(element, setting) {
        this.#addGenericSetting(element, setting, "text");
    }

    #addColorSetting(element, setting) {
        this.#addGenericSetting(element, setting, "color");
    }

    async #write(id, value) {
        try {
            await this.#provider.write(id, value);
            this.#handleKnownSetting(id, value);
        }
        catch (ex) {
            console.error("failed to write setting", id, ex);
        }
    }

    #handleKnownSetting(id, value) {
        switch (id) {
            case "view.titlebar.color":
                setColor(".titlebar", value);
                break;
            case "notes.path":
                this.#oninit();
                break;
            default:
                break;
        }
    }
    
}

function init_settings(provider, notelist, taglist) {
    new SettingsPage(provider, notelist, taglist);
};

export { init_settings }