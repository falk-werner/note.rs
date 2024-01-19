
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

const init_settings = () => {
    const titlebar = document.querySelector("#settings_titlebar");
    titlebar.addEventListener("input", () => {
        const rule = get_rule(".titlebar");
        rule.style.background=titlebar.value;
    });
};

export { init_settings }