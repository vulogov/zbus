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

pub fn zabbix_key_to_zenoh(hostid: String, itemid: String, key: String) -> Option<String> {
    let mut res = String::from(format!("zbus/metadata/v1/{}/{}", hostid, itemid).to_string());
    let mut lex = ZabbixKeyToken::lexer(&key);
    loop {
        match lex.next() {
            Some(Ok(ZabbixKeyToken::Ident)) => {
                res = [res, "/".to_string(), (&lex.slice()).to_string()].join("");
            }
            Some(Ok(ZabbixKeyToken::Lenc)) => {
                break;
            }
            Some(Err(err)) => {
                log::warn!("Error converting Zabbix key {}: {:?}", &lex.slice(), err);
                return None;
            }
            None => break,
            _ => continue,
        }
    }
    Some(res)
}
