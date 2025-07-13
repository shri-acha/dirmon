mod internals;
use internals::*;

fn main() {
    let extensions = vec!["png".to_string(), "mp3".to_string()];
    let dir = "/home/shri/Downloads/Telegram Desktop/";

    let my_dirmon_dir = DirMonDirectory {
        w_dir: dir.to_string(),
        w_extns: extensions,
    };
    let my_dirmon_conf = DirMonConfig {
        w_dirmon: vec![my_dirmon_dir],
    };
    let my_dirmon_inst = DirMonInstance{
        w_dirmon_conf:my_dirmon_conf,
    };

    let _ = my_dirmon_inst.start_listening();

}
