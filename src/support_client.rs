use hbb_common::config;

const APP_NAME: &str = "Mo3en";
const SERVER: &str = "209.209.41.243";
const KEY: &str = "fK1MG5ucCY2AF51NVKXiUtQJS9ZOq4I13khsDL6HIB0=";

// Built-in settings for the incoming-only support client, applied without a signed custom.txt.
// APP_NAME must match the exe name inside the package (mo3en.exe) and contain no spaces:
// it is also used for the exe lookup, settings folder and URI scheme.
pub fn apply() {
    *config::APP_NAME.write().unwrap() = APP_NAME.to_owned();

    let mut hard = config::HARD_SETTINGS.write().unwrap();
    for (k, v) in [
        ("conn-type", "incoming"),
        ("disable-settings", "Y"),
        ("disable-installation", "Y"),
        ("disable-ab", "Y"),
        ("disable-account", "Y"),
    ] {
        hard.insert(k.to_owned(), v.to_owned());
    }
    drop(hard);

    let mut overwrite = config::OVERWRITE_SETTINGS.write().unwrap();
    for (k, v) in [
        ("custom-rendezvous-server", SERVER),
        ("relay-server", SERVER),
        ("api-server", ""),
        ("key", KEY),
        ("approve-mode", "click"),
        ("allow-auto-update", "N"),
    ] {
        overwrite.insert(k.to_owned(), v.to_owned());
    }
    drop(overwrite);

    config::OVERWRITE_LOCAL_SETTINGS
        .write()
        .unwrap()
        .insert("enable-check-update".to_owned(), "N".to_owned());

    let mut builtin = config::BUILTIN_SETTINGS.write().unwrap();
    for k in [
        "hide-security-settings",
        "hide-network-settings",
        "hide-server-settings",
        "hide-proxy-settings",
        "hide-help-cards",
        "hide-stop-service",
        "disable-change-id",
        "disable-change-permanent-password",
    ] {
        builtin.insert(k.to_owned(), "Y".to_owned());
    }
}
