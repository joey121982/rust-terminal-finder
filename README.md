# rust-terminal-finder

## Version 0.2.1

Command line program that finds a specific file/folder and displays its location. written in rust.

### Usage:
* rust-terminal-finder <br>
    --name (or -n) \<NAME> <br>
    --ftype (or -f) \<fi / fo> (OPTIONAL, if not specified, searches for both) <br>
    ^ where ```fi``` is for file type and ```fo``` is for folder type.<br>
    --location (or -l) \<LOCATION> (OPTIONAL, if not specified, goes to default)<br>
    ^ current default is ```/home```<br>

### NOTE: as of 0.2.1, the program uses tokio async to speed up the process.