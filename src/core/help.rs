use std::fs;
use std::path::Path;

pub fn display_help(first_time: bool) {
    if first_time {
        println!("
        Welcome to blabar - A CLI tool to manage fediverse instances!

        USAGE:
            blabar [OPTIONS]

        OPTIONS:
            -h, --help       Show this help message
            -a, --add        Add a fedi instance
            -m, --manege     Manege all fedi instances
            -c, --connect    Connect the client to a server running blastation.
            -u, --update     Update instances in one command remotely.

        MORE INFO:

        © 2024 chlove under the GNU GPL 3.0 licence

        ");
    } else {
        println!("
        blabar - A CLI tool to manage fediverse instances!

        USAGE:
            blabar [OPTIONS]

        OPTIONS:
            -h, --help       Show this help message
            -a, --add        Add a fedi instance
            -m, --manege     Manege all fedi instances
            -c, --connect    Connect the client to a server running blastation.
            -u, --update     Update instances in one command remotely.

        MORE INFO:

        © 2024 chlove under the GNU GPL 3.0 licence
        
        ");
    }
}

pub fn is_first_time() -> bool {
    let config_path = dirs::home_dir().unwrap().join(".blabar_config");

    if !Path::new(&config_path).exists() {
        let _ = fs::File::create(&config_path);
        return true;
    }
    false
}
