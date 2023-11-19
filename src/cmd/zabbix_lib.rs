extern crate log;
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[, \t\n\f]+")]
enum ZabbixKeyToken {
    #[regex("(\")*[0-9a-zA-Z./_-]+(\")*")]
    Ident,
    #[token("[")]
    Renc,
    #[token("]")]
    Lenc,
}

pub fn zabbix_key_to_zenoh_meta(hostid: String, itemid: String, key: String) -> Option<String> {
    match zabbix_key_to_zenoh(key.clone()) {
        Some(zkey) => {
            return Some(format!("zbus/metadata/v1/{}/{}{}", hostid, itemid, zkey).to_string());
        }
        None => return None,
    }
}

pub fn zabbix_key_to_zenoh(key: String) -> Option<String> {
    log::trace!("Parsing: {:?}", &key);
    let mut res = String::from("".to_string());
    let mut lex = ZabbixKeyToken::lexer(&key);
    loop {
        match lex.next() {
            Some(Ok(ZabbixKeyToken::Ident)) => {
                let mut val = (&lex.slice()).to_string();
                log::trace!("Got ident: {:?}", &val);
                if val.is_empty() {
                    val = "_".to_string();
                } else {
                    val = val.replace("/", "\\");
                }
                res = [res, "/".to_string(), val].join("");
            }
            Some(Ok(ZabbixKeyToken::Lenc)) => {
                break;
            }
            Some(Err(err)) => {
                log::warn!("Error converting Zabbix key {}: {:?}", &lex.slice(), err);
                return None;
            }
            None => break,
            Some(Ok(something)) => {
                log::trace!("Got something: {:?} {:?}", something, (&lex.slice()).to_string());
            }
        }
    }
    Some(res)
}
