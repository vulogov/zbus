extern crate log;
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[, \t\n\f]+")]
enum PrometheusKeyToken {
    #[regex("(\")*[0-9a-zA-Z./_-]+(\")*")]
    Ident,
    #[token("{")]
    Renc,
    #[token("}")]
    Lenc,
}

pub fn prometheus_key_to_zenoh(s: &prometheus_parse::Sample) -> String {
    let mut res = String::from(s.metric.to_string());
    for (l,v) in s.labels.iter() {
        let val = format!("{}:{}", &l, &v);
        res = [res, "/".to_string(), val.replace("/", "\\")].join("");
    }
    log::trace!("Parsing: {:?} to {:?}", &s.metric, &res);
    res
}
