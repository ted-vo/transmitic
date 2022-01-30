use std::env;
use std::error::Error;
use std::f64::consts::FRAC_1_PI;
use std::path::Path;
use std::process::Command;
use std::str;

use serde::{Deserialize, Serialize};
extern crate sciter;
use sciter::dispatch_script_call;
use sciter::Value;
use transmitic_core::incoming_uploader::SharingState;
use transmitic_core::shared_file::SelectedDownload;
use transmitic_core::transmitic_core::TransmiticCore;

const VERSION: &str = "0.10.0"; // Note: And cargo.toml
const NAME: &str = "Transmitic Beta";
const URL: &str = "https://transmitic.net";

struct Handler {
    transmitic_core: TransmiticCore,
}

#[derive(Serialize, Deserialize, Debug)]
struct FilesJson {
    files: Vec<PathJson>,
}

#[derive(Serialize, Deserialize, Debug)]
struct PathJson {
    path: String,
}

// TODO escape all strings for HTML GUI !!!

impl Handler {
    fn add_files(&mut self, files: Value) -> Value {
        let mut clean_strings: Vec<String> = Vec::new();
        if files.is_string() {
            let clean_file = self.clean_sciter_string(files);
            clean_strings.push(clean_file);
        }
        else {
            for file in files.into_iter() {
                let clean_file = self.clean_sciter_string(file);
                clean_strings.push(clean_file);
            }
        }

        let response: Value;
        match self.transmitic_core.add_files(clean_strings) {
            Ok(_) => response = self.get_msg_box_response(0, &"".to_string()),
            Err(e) => response = self.get_msg_box_response(1, &e.to_string()),
        }
        return response;
    }

    fn add_new_user(&mut self, new_nickname: Value, new_public_id: Value, new_ip: Value, new_port: Value) -> Value {
        let new_nickname = self.clean_sciter_string(new_nickname);
        let new_public_id = self.clean_sciter_string(new_public_id);
        let new_ip = self.clean_sciter_string(new_ip);
        let new_port = self.clean_sciter_string(new_port);

        let response: Value;
        match self.transmitic_core.add_new_user(new_nickname, new_public_id, new_ip, new_port) {
            Ok(_) => response = self.get_msg_box_response(0, &"".to_string()),
            Err(e) => response = self.get_msg_box_response(1, &e.to_string()),
        }
        return response;
    }

    fn add_user_to_shared(&mut self, nickname: Value, file_path: Value) -> Value {
        let nickname = self.clean_sciter_string(nickname);
        let mut file_path = self.clean_sciter_string(file_path);
        file_path = file_path.replace("\\\\", "\\"); // TODO stdlib function for normalizing file paths?

        let response: Value;
        match self.transmitic_core.add_user_to_shared(nickname, file_path) {
            Ok(_) => response = self.get_msg_box_response(0, &"".to_string()),
            Err(e) => response = self.get_msg_box_response(1, &e.to_string()),
        }
        return response;
    }

    fn create_new_id(&mut self) -> Value {
        let response: Value;
        match self.transmitic_core.create_new_id() {
            Ok(_) => response = self.get_msg_box_response(0, &"".to_string()),
            Err(e) => response = self.get_msg_box_response(1, &e.to_string()),
        }
        return response;
    }

    fn download_selected(&mut self, files: Value) -> Value {
        println!("download selected");
        let files = files.get_item("files");

        println!("{:?}", files);
        let mut downloads: Vec<SelectedDownload> = Vec::new();
        for file in files.values() {
            println!("{:?}", file);
            let owner = self.clean_sciter_string(file.get_item("owner"));
            let path = self.clean_sciter_string(file.get_item("path")).replace("\\\\", "\\");
            let new_download = SelectedDownload {
                path,
                owner,
            };
            downloads.push(new_download);
        };
        
        println!("{:?}", downloads);

        let response: Value;
        match self.transmitic_core.download_selected(downloads) {
            Ok(_) => response = self.get_msg_box_response(0, &"Files will be downloaded".to_string()),
            Err(e) => response = self.get_msg_box_response(1, &e.to_string()),
        }
        return response;
    }

    fn downloads_open(&self) {}

    fn downloads_clear_finished(&self) {}

    fn downloads_clear_finished_from_me(&self) {}

    fn downloads_cancel_invalid(&self) {}

    fn downloads_cancel_all(&self) {}

    fn downloads_pause_all(&self) {}

    fn downloads_resume_all(&self) {}

    fn get_downloads_in_progress(&self) -> Value {
        let mut response = Value::new();

        let mut item1 = Value::new();
        item1.set_item("owner", "My Mock");
        item1.set_item("percent", "90");
        item1.set_item("path", "C:\\users\\other\\hello.txt");

        let mut item2 = Value::new();
        item2.set_item("owner", "My Mock2");
        item2.set_item("percent", "5");
        item2.set_item("path", "C:\\users\\other\\hello3.txt");

        response.push(item1);
        response.push(item2);

        return response;
    }

    fn get_msg_box_response(&self, code: i32, msg: &String) -> Value {
        let mut response = Value::new();

        response.push(Value::from(code));
        response.push(Value::from(msg));
        response
    }

    fn clean_sciter_string(&self, s: Value) -> String {
        let mut s = s.to_string();
        s = s[1..s.len() - 1].to_string();
        s = s.trim().to_string();
        s
    }

    fn open_a_download(&self, file_path: Value) {
        let mut file_path = self.clean_sciter_string(file_path);
        file_path = file_path.replace("\\\\", "\\");
        println!("Open a download {}", file_path);
        let p = Path::new(&file_path);
        let dir_path = p.parent().unwrap();
        Command::new("explorer.exe").arg(dir_path).spawn();
    }

    fn get_app_display_name(&self) -> Value {
        Value::from(NAME)
    }

    fn get_app_display_version(&self) -> Value {
        Value::from(format!("v{}", VERSION))
    }

    fn get_app_url(&self) -> Value {
        Value::from(URL)
    }

    fn get_local_ip(&self) -> Value {
        // TODO
        Value::from("192.168.X.X")
    }

    fn get_my_sharing_files(&self) -> Value {
        let my_sharing_files = self.transmitic_core.get_my_sharing_files();
        let mut shared_users = Vec::new();
        for user  in self.transmitic_core.get_shared_users() {
            shared_users.push(user.nickname);
        }

        // DO Use struct and serde to clean this up
        let mut my_files = Value::array(0);
        for file in my_sharing_files {
            let mut new_file = Value::new();
            new_file.set_item("file_path", file.path);

            let mut shared_with = Value::array(0);
            for s in file.shared_with.iter() {
                shared_with.push(s);
            }
            new_file.set_item("shared_with", shared_with);

            let mut add_users = Value::array(0);
            for user in shared_users.iter() {
                if !file.shared_with.contains(user) {
                    add_users.push(user.clone());
                }
            }
            new_file.set_item("add_users", add_users);

            my_files.push(new_file);
        }

        return my_files;
    }

    fn get_my_sharing_state(&self) -> Value {
        let state = match self.transmitic_core.get_my_sharing_state() {
            SharingState::Off => "Off".to_string(),
            SharingState::Local => "Local".to_string(),
            SharingState::Internet =>"Internet".to_string(),
        };
        return self.get_msg_box_response(0, &state);
    }

    fn get_shared_users(&self) -> Value {
        let shared_users = self.transmitic_core.get_shared_users();
        let mut user_list = Value::new();

        for user in shared_users {
            let mut new_user_dict = Value::new();
            // TODO use serde to get the string? 
            //  But the struct may evenutally contain data that the UI doesn't need?
            //  But that would just be ignored, that wouldn't be a problem

            new_user_dict.set_item("nickname", user.nickname);
            new_user_dict.set_item("public_id", user.public_id);
            new_user_dict.set_item("ip", user.ip);
            new_user_dict.set_item("port", user.port);
            if user.allowed {
                new_user_dict.set_item("status", "Allowed");
            } else {
                new_user_dict.set_item("status", "Blocked");
            }
            
            user_list.push(new_user_dict);
            
        }

       return user_list;
    }

    fn get_sharing_port(&self) -> Value {
        let sharing_port = self.transmitic_core.get_sharing_port();
        //return self.get_msg_box_response(0, &sharing_port);
        return Value::from(sharing_port);
    }

    fn get_public_id_string(&self) -> Value {
        let public_id_string = self.transmitic_core.get_public_id_string();
        Value::from(public_id_string)
    }

    fn refresh_shared_with_me(&self) {}

    fn remove_file_from_sharing(&mut self, file_path: Value) -> Value {
        let mut file_path = self.clean_sciter_string(file_path);
        file_path = file_path.replace("\\\\", "\\");
        
        let response;
        match self.transmitic_core.remove_file_from_sharing(file_path) {
            Ok(_) => response = self.get_msg_box_response(0, &"".to_string()),
            Err(e) => response = self.get_msg_box_response(1, &e.to_string()),
        }

        return response;
    }

    fn remove_user(&mut self, nickname: Value) -> Value {
        let nickname = self.clean_sciter_string(nickname);
        let response;
        match self.transmitic_core.remove_user(nickname) {
            Ok(_) => response = self.get_msg_box_response(0, &"".to_string()),
            Err(e) => response = self.get_msg_box_response(1, &e.to_string()),
        }

        return response;
    }

    fn remove_user_from_sharing(&mut self, nickname: Value, file_path: Value) -> Value {
        let nickname = self.clean_sciter_string(nickname);
        let mut file_path = self.clean_sciter_string(file_path);

        file_path = file_path.replace("\\\\", "\\");

        let response: Value;
        match self.transmitic_core.remove_user_from_sharing(nickname, file_path) {
            Ok(_) => response = self.get_msg_box_response(0, &"".to_string()),
            Err(e) => response = self.get_msg_box_response(1, &e.to_string()),
        }

        return response;
    }

    fn set_my_sharing_state(&mut self, state: Value) -> Value {
        let state = self.clean_sciter_string(state);

        let response: Value;
        let core_state: SharingState;
        if state == "Off" {
            core_state = SharingState::Off;
        }
        else if state == "Local Network" {
            core_state = SharingState::Local;
        }
        else if state == "Internet" {
            core_state = SharingState::Internet;
        }
        else {
            return self.get_msg_box_response(1, &format!("Sharing state '{}' is not valid", state));
        }

        self.transmitic_core.set_my_sharing_state(core_state);
        return self.get_msg_box_response(0, &"".to_string());
    }

    fn set_port(&mut self, port: Value) -> Value {
        let port = self.clean_sciter_string(port);

        let response: Value;
        match self.transmitic_core.set_port(port) {
            Ok(_) => response = self.get_msg_box_response(0, &"".to_string()),
            Err(e) => response = self.get_msg_box_response(1, &e.to_string()),
        }
        
        return response;
    }

    fn set_user_is_allowed_state(&mut self, nickname: Value, is_allowed: Value) -> Value {
        let nickname = self.clean_sciter_string(nickname);
        let is_allowed = match is_allowed.to_bool() {
            Some(is_allowed) => is_allowed,
            None => return self.get_msg_box_response(1, &"is_allowed is not a bool".to_string()),
        };

        let response: Value;
        match self.transmitic_core.set_user_is_allowed_state(nickname, is_allowed) {
            Ok(_) => response = self.get_msg_box_response(0, &"".to_string()),
            Err(e) => response = self.get_msg_box_response(1, &e.to_string()),
        }
        
        return response;
    }

    fn update_user(
        &mut self,
        nickname: Value,
        new_public_id: Value,
        new_ip: Value,
        new_port: Value,
    ) -> Value {
        let nickname = self.clean_sciter_string(nickname);
        let new_public_id = self.clean_sciter_string(new_public_id);
        let new_ip = self.clean_sciter_string(new_ip);
        let new_port = self.clean_sciter_string(new_port);

        let response: Value;
        match self.transmitic_core.update_user(nickname, new_public_id, new_ip, new_port) {
            Ok(_) => response = self.get_msg_box_response(0, &"".to_string()),
            Err(e) => response = self.get_msg_box_response(1, &e.to_string()),
        }

        return response;
    }
}

impl sciter::EventHandler for Handler {
    dispatch_script_call! {
        fn add_files(Value);
        fn add_new_user(Value, Value, Value, Value);
        fn add_user_to_shared(Value, Value);
        fn create_new_id();
        fn download_selected(Value);
        fn downloads_open();
        fn downloads_clear_finished();
        fn downloads_clear_finished_from_me();
        fn downloads_cancel_invalid();
        fn downloads_cancel_all();
        fn downloads_pause_all();
        fn downloads_resume_all();

        fn refresh_shared_with_me();
        fn remove_file_from_sharing(Value);
        fn remove_user(Value);
        fn remove_user_from_sharing(Value, Value);

        fn get_app_display_name();
        fn get_app_display_version();
        fn get_app_url();

        fn get_downloads_in_progress();
        fn get_local_ip();
        fn get_my_sharing_files();
        fn get_my_sharing_state();
        fn get_shared_users();
        fn get_sharing_port();
        fn get_public_id_string();

        fn open_a_download(Value);

        fn set_my_sharing_state(Value);
        fn set_port(Value);
        fn set_user_is_allowed_state(Value, Value);
        fn update_user(Value, Value, Value, Value);

    }
}

fn main() {
    println!("{} v{}", NAME, VERSION);
    println!("{}", URL);
    let args: Vec<String> = env::args().collect();
    println!("cli args");
    println!("{:?}\n", args);

    let mut sciter_path = env::current_dir().unwrap();
    sciter_path.push("transmitic\\src\\main.htm");
    let sciter_string = format!("file://{}", sciter_path.to_string_lossy());

    println!("Current Working Dir: '{:?}'", env::current_dir().unwrap());
    println!("Sciter path: '{}'", sciter_string);
    println!("Transmitic Path: '{:?}'", env::current_exe().unwrap());

    let mut frame = get_sciter_frame();

    let transmitic_core: TransmiticCore;
    match TransmiticCore::new() {
        Ok(t) => {
            transmitic_core = t;
        }
        Err(e) => {
            let html_error = format!("Transmitic failed to start<br>{}", &e.to_string());
            frame.load_html(&html_error.into_bytes(), Some("example://main.htm"));
            frame.run_app();
            panic!("{:?}", e.to_string());
        }
    }

    let mut handler = Handler { transmitic_core };

    frame.event_handler(handler);
    frame.load_file(&sciter_string);
    frame.run_app();
}

fn get_sciter_frame() -> sciter::Window {
    sciter::set_options(sciter::RuntimeOptions::ScriptFeatures(
        sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_SYSINFO as u8
            | sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_FILE_IO as u8,
    ))
    .unwrap();
    sciter::set_options(sciter::RuntimeOptions::DebugMode(true)).unwrap();

    let frame = sciter::Window::new();

    if cfg!(target_os = "macos") {
        frame
            .set_options(sciter::window::Options::DebugMode(true))
            .unwrap();
    }

    return frame;
}
