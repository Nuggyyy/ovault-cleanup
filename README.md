# OVault-Cleanup

This is a little tool that helps cleaning up the attachments directory in an obsidian vault, built in Rust.

## How It Works
- It first comprises a list of all attachments in the command-line argument specified attachments folder (--attachments-dir).
- Then, it builds the regex pattern from that attachments list, which it matches against all files in the rest of the vault, keeping track which patterns it found.
- from the list of matches, it then builds the unmatched list which is passed to a delete function
- the delete function asks for human input for each file it wants to delete, only deleting if "y" was answered, and skipping for any other input (including hitting the Enter key)

## Use
I recommend copying the binary file into the obsidian vault for easy use.
Two command-line arguments are required:
- --attachments-dir: it specifies where the attachments directory is and must end with a "/" for MacOS/Linux or "\\" for Windows, otherwise the delete function won't find the file
- --vault: it specifies where the vault directory is. in most cases the vault directory is the same from which the tool is called, meaning a simple "." suffices as its argument

## Future
Since this project is quite young, there are many ways this tool can be extended. For example, an optional flag that showcases all folders that were traversed so that you can ensure that the tool looked at every relevant document can be implemented. Making the --vault argument optional and establishing "." as the default could be beneficial. There are many ways to improve this tool, which I plan on doing.
