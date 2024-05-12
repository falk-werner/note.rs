class SettingsProvider {

    /**
     * Returns a list of all settings.
     * 
     * @returns {Setting[]} List of settings.
     */
    async readAll() {
        throw new Error("Not implemented");
    }

    /**
     * Sets the value of single setting.
     * 
     * @param {*} name name of the setting
     * @param {*} value new value of the setting
     * @throws {Error} unknown setting
     */
    async write(name, value) {
        throw new Error("Not implemented");
    }
}

export { SettingsProvider }