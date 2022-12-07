use std::io;
use std::collections::HashMap;


#[derive(Debug)]
enum FS {
    Dir(String),
    File(String, usize),
}

fn main() {
    let fs = parse();
    let mut ans = 0;
    for path in fs.keys() {
        let size = get_size(path, &fs);
        if size <= 100000 {
            ans += size;
        }
        println!("{} {}", path, size);
    }
    println!("{}", ans);
}

fn get_size(path: &str, fs: &HashMap<String, Vec<FS>>) -> usize {
    let mut size: usize = 0;
    for child in fs.get(path).unwrap() {
        match child {
            FS::File(_, s) => size += s,
            FS::Dir(name) => {
                let mut this_path = path.to_string();
                if this_path != "/" {
                    this_path.push('/');
                }
                this_path.push_str(name);
                size += get_size(&this_path, fs);
            },
        };
    }
    size
}

fn parse() -> HashMap<String, Vec<FS>> {
    let mut fs: HashMap<String, Vec<FS>> = HashMap::new();

    let mut pwd: Vec<String> = Vec::new();
    for l in io::stdin().lines() {
        let line = l.unwrap();
        let v: Vec<&str> = line.split(' ').collect();
        match v[..] {
            ["$", "cd", "/"] => {
                pwd.clear();
                update_fs(&pwd, &mut fs);
            },
            ["$", "cd", ".."] => {
                pwd.pop();
            },
            ["$", "cd", name] => {
                pwd.push(name.to_string());
                update_fs(&pwd, &mut fs);
            },
            ["$", "ls"] => {},
            ["dir", name] => {
                let mut path = to_full_path(&pwd);
                let fs_list = fs.get_mut(&path).unwrap();
                path.push('/');
                path.push_str(name);
                let fs = FS::Dir(name.to_string());
                fs_list.push(fs);
            },
            [size, name] => {
                let mut path = to_full_path(&pwd);
                let fs_list = fs.get_mut(&path).unwrap();
                path.push('/');
                path.push_str(name);
                let fs = FS::File(
                    name.to_string(),
                    size.parse().unwrap()
                );
                fs_list.push(fs);
            },
            _ => panic!("unknown input {}", line),
        }
    }

    return fs;
}

fn to_full_path(pwd: &Vec<String>) -> String {
    if pwd.is_empty() {
        return "/".to_string();
    }
    let mut ret = String::new();
    for p in pwd {
        ret.push('/');
        ret.push_str(p);
    }
    return ret;
}

fn update_fs(pwd: &Vec<String>, fs: &mut HashMap<String, Vec<FS>>) {
    let path = to_full_path(&pwd);
    if !fs.contains_key(&path) {
        fs.insert(
            path,
            Vec::new(),
        );
    }
}
