//! Windows registry read helpers for Clip Studio Paint Debut.

pub const APP_KEY: &str = "Software\\Clip\\Clip Studio Paint Debut";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Clip Studio Paint Debut";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Clip Studio Paint Debut") },
        RegValue { name: "Version",     value: String::from("v1.7.5") },
        RegValue { name: "Channel",     value: String::from("beta") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Clip Studio Paint Debut") },
        RegValue { name: "Publisher",       value: String::from("Clip") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Clip Studio Paint Debut\\uninstall.exe\"") },
    ]
}
