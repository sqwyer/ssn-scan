extern crate tinyfiledialogs as tfd;
extern crate web_view;

use tfd::MessageBoxIcon;
use web_view::*;
use walkdir::WalkDir;

use std::{path::{Path}, fs::{File, metadata}, io::Read, collections::HashMap};
use regex::Regex;

pub const MATCH: &str = r"(\d{9}+)|(\d{3}-\d{2}-\d{4}+)";

pub struct LineResult {
    pub numbers: Vec<String>,
    pub count: u128
}

pub fn read_file(path: &Path) -> Vec<u8> {
    let mut file_content = Vec::new();
    let mut file = File::open(&path).expect("Unable to open file");
    file.read_to_end(&mut file_content).expect("Unable to read");
    file_content
}

pub fn scan_text(text: &str) -> HashMap::<u128,LineResult> {
    let regex_str = Regex::new(MATCH).unwrap();
    let mut all_lines: HashMap<u128, LineResult> = HashMap::new();
    let mut line_num: u128 = 0;
    for line in text.lines() {
        line_num += 1;
        let mut numbers = vec![];
        for cap in regex_str.captures_iter(line) {
            numbers.push(cap.get(0).map_or("".to_string(), |m| m.as_str().to_string()));
        }
        let count = u128::try_from(numbers.len()).unwrap();
        let line_res = LineResult {
            numbers: numbers,
            count: count
        };
        all_lines.insert(line_num, line_res);
    }
    all_lines
}

pub fn scan_file(path: &Path) -> Option<HashMap::<u128,LineResult>> {
    let buf = read_file(&path);
    let s = match std::str::from_utf8(&buf) {
        Ok(v) => v,
        Err(_) => return None,
    };
    Some(scan_text(s))
}

pub(crate) fn make() -> WVResult {
    let webview = web_view::builder()
        .title("Dialog example")
        .content(Content::Html(HTML))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|webview, arg| {
            match arg {
                "open_folder" => match tfd::select_folder_dialog("Please choose a folder...", "") {
                    Some(path) => {
                        tfd::message_box_ok("Folder chosen", &path, MessageBoxIcon::Info);
                        for file in WalkDir::new(&path).into_iter().filter_map(|file| file.ok()) {
                        let m_path = &file.path();
                        if metadata(m_path).unwrap().is_file() {
                            let result = scan_file(&m_path);
                            if result.is_none() {
                                continue;
                            }
                            match result {
                                Some(lines) => {
                                    // println!("made it here: {:?}", &m_path.to_str());
                                    let mut l_possible: usize = 0;
                                    // let mut line_num = 0;
                                    for line in lines.iter() {
                                        // line_num+=1;
                                        l_possible+=line.1.numbers.len();
                                    }
                                    webview.eval(&format!("populate(['{:?}',{},'{:?}'])", m_path.as_os_str(), l_possible,path.as_str())).unwrap();
                                },
                                None => continue,
                            }
                        }
                    }
                    },
                    None => tfd::message_box_ok(
                        "Warning",
                        "You didn't choose a folder.",
                        MessageBoxIcon::Warning,
                    ),
                },
                // "open_file" => match tfd::open_file_dialog("Please choose a file...", "", None) {
                //     Some(path) => tfd::message_box_ok("File chosen", &path, MessageBoxIcon::Info),
                //     None => tfd::message_box_ok(
                //         "Warning",
                //         "You didn't choose a file.",
                //         MessageBoxIcon::Warning,
                //     ),
                // }
                // "pre_scanned" => tfd::message_box_ok(
                //     "Warning",
                //     "That folder has already be scanned, this may cause some failures.",
                //     MessageBoxIcon::Warning,
                // ),
                "exit" => webview.exit(),
                _ => unimplemented!(),
            };
            Ok(())
        })
        .build()?;

    webview.run()
}

const HTML: &str = r#"
<!doctype html>
<html>
    <head>
        <style>
        * {
            padding: 0;
            margin: 0;
        }
        body {
            font-family: sans-serif;
            padding: 1em;
        }
        button {
            background: #32B6FF;
            color: #fff;
            border: none;
            border-radius: 4px;
            padding: 1em;
            cursor: pointer;
        }
        button:hover {
            background: #3BADED;
            transition: 0.1s ease;
        }
        button.danger {
            background: #FF4E4E;
        }
        button.danger:hover {
            background: #ED3B3B;
        }
        #content {
            margin: 1em 0;
        }
        .file {
            border-bottom: 1.5px solid #eee;
            padding: 1em 0;
        }
        .file .main {
            display: flex;
            flex-direction: row;
            gap: 0.5em;
            align-items: center;
        }
        .file .main label {
            font-weight: bold;
        }
        .file .main p {
            color: #505050;
            margin-left: 10px;
        }
        .file .main span {
            color: #303030;
            cursor: pointer;
            text-decoration: underline;
            margin-left: auto;
        }
        .file .hidden {
            display: none;
        }
        </style>
    </head>
    <body>
        <button onclick="external.invoke('open_folder')">Scan Folder</button>
        <!--<button onclick="external.invoke('open_file')">Scan File</button>-->
        <div id="content">
            <!--No File or Folder Unselected-->
            <!--<div class="file">
                <div class="main">
                    <label>/file/path.txt</label>
                    <p>11 Possible Total</p>
                    <span onclick="ext(this)">Show Extended</span>
                </div>
                <div class="extended hidden">
                ...
                </div>
            </div>-->
        </div>
        <button class="danger" onclick="external.invoke('exit')">Exit</button>
        <script>
            let scans = [];
            let paths = [];
            function ext(self) {
                let extra = self.parentNode.parentNode.children.item(1);
                extra.classList.toggle('hidden');
                if (extra.classList.contains('hidden')) {
                    self.parentNode.children.item(2).innerHTML = 'Show Extended'
                } else {
                    self.parentNode.children.item(2).innerHTML = 'Hide Extended'
                }
            }
            function fix(s) {
                s.replace
                return s.replaceAll('"','');
            }
            function populate(content) {
                content[0] = fix(content[0]);
                content[2] = fix(content[2]);
                if(!paths.includes(content[0])) paths.push(content[0])
                let file = document.createElement('div');
                file.className = 'file';
                let main = document.createElement('div');
                main.className = 'main';
                let main_label = document.createElement('label');
                main_label.innerHTML = content[0].replace(content[2],'');
                let main_possible = document.createElement('p');
                main_possible.innerHTML = `${content[1]} Possible SSNs`;
                let main_ext = document.createElement('span');
                main_ext.setAttribute('onclick', 'ext(this)');
                main_ext.innerHTML = 'Show Extended';
                main.appendChild(main_label);
                main.appendChild(main_possible);
                main.appendChild(main_ext);
                let extra = document.createElement('div');
                extra.className = 'extended hidden';
                extra.innerHTML = '...';
                file.appendChild(main);
                file.appendChild(extra);
                let mcontent = document.getElementById('content');
                // mcontent.innerHTML = '';
                if (!scans.includes(content[2])) {
                    let pathname = document.createElement('h3');
                    pathname.innerHTML = `Scanned: "${content[2]}"`;
                    mcontent.innerHTML += '<br />'
                    mcontent.appendChild(pathname);
                    mcontent.innerHTML += '<br />'
                    scans.push(content[2]);
                }
                mcontent.appendChild(file);
            }
        </script>
    </body>
</html>
"#;