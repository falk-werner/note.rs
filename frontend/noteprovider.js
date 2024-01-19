
/**
 * This class documents the NoteProvider interface.
 */
class NoteProvider {

    /**
     * Returns a list of the names of all notes.
     * 
     * @returns {String[]} List containing the names of all notes.
     */
    async list() {
        throw new Error("Not implementd");
    }

    /**
     * Returns the contents of a note.
     * 
     * @param {String} name Name of the the note.
     * @returns {String} contents of the note
     * @throws {Error} unknown note
     */
    async read(name) {
        throw new Error("Not implementd");
    }

    /**
     * Creates a new note and returns it's name.
     * 
     * @returns {String} Name of the newly created note.
     */
    async create() {
        throw new Error("Not implementd");
    }

    /**
     * Renames an existing note.
     * 
     * @param {String} old_name Name of an existing note
     * @param {String} new_name New name of the note
     * @throws {Error} unknown note
     * @throws {Error} there is already a note with the new name
     */
    async rename(old_name, new_name) {
        throw new Error("Not implementd");
    }

    /**
     * Writes the contents of an existing note.
     * 
     * @param {String} name Name of the note 
     * @param {String} content New contents of the note
     * @throws {Error} unknown note
     */
    async write(name, content) {
        throw new Error("Not implementd");
    }    

    /**
     * Removes an existing note.
     * 
     * Note that this method does not fail, if
     * the note does not exist.
     * 
     * @param {String} name 
     */
    async remove(name) {
        throw new Error("Not implementd");
    }

    /**
     * Returns all tags of an existing note. 
     * 
     * @param {String} name name of then note
     * @returns {String[]} tags of the note
     * @throws {Error} unknown note
     */
    async read_tags(name) {
        throw new Error("Not implementd");
    }

    /**
     * Writes (replaces) tht tags of an existing note
     * 
     * @param {String} name name of the note
     * @param {String[]} tags tags of the note
     * @throws {Error} unknown note
     */
    async write_tags(name, tags) {
        throw new Error("Not implementd");
    }

    /**
     * Takes a screenshot and returns its filename.
     * 
     * @param {String} name name of the note associated with the screenshot
     * @returns {String} filename of the screenshot (relative to the note's directory)
     * @throws {Error} unknown note
     * @throws {Error} screenshot command failed
     */
    async take_screenshot(name) {
        throw new Error("Not implemented");
    }

    /**
     * Opens the note directory in a file browser.
     * 
     * @param {String} name name of the note associated with the screenshot
     * @throws {Error} unknown note
     */
    async open_note_directory(name) {
        throw new Error("Not implemented");
    }
}


export { NoteProvider }