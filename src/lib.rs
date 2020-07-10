use reqwest;
use select::document::Document;
use select::predicate::Attr;
use select::predicate::Class;

#[derive(Debug)]
pub struct Verb {
    pub present: String,
    pub past_simple: String,
    pub past_part: String,
}

pub fn lookup(s: &str) -> Verb {
    let head_and_tail = s.splitn(2, " ").collect::<Vec<_>>();
    let head = head_and_tail[0].to_string();
    let mut tail: String = String::new();
    if head_and_tail.len() > 1 {
        tail = format!(" {}", head_and_tail[1]);
    }

    let url = format!(
        "https://www.oxfordlearnersdictionaries.com/definition/english/{}_1",
        head
    );
    let resp = reqwest::blocking::get(&url).unwrap();
    let doc = Document::from_read(resp).unwrap();

    match doc.find(Class("verb_forms_table")).next() {
        None => {
            let past_simple = match &head {
                hd if hd.ends_with("e") => format!("{}d{}", hd, tail),
                _ => format!("{}ed{}", head, tail),
            };
            let past_part = past_simple.clone();
            return Verb {
                present: s.to_string(),
                past_simple,
                past_part,
            };
        }
        Some(table) => {
            let past_simple = match table.find(Attr("form", "past")).next() {
                Some(ps) => format!(
                    "{}{}",
                    ps.text().split_whitespace().collect::<Vec<_>>()[2],
                    tail
                ),
                None => match &head {
                    hd if hd.ends_with("e") => format!("{}d{}", hd, tail),
                    _ => format!("{}ed{}", head, tail),
                },
            };
            let past_part = match table.find(Attr("form", "pastpart")).next() {
                Some(ps) => format!(
                    "{}{}",
                    ps.text().split_whitespace().collect::<Vec<_>>()[2].to_string(),
                    tail
                ),
                None => match &head {
                    hd if hd.ends_with("e") => format!("{}d{}", hd, tail),
                    _ => format!("{}ed{}", head, tail),
                },
            };
            return Verb {
                present: s.to_string(),
                past_simple,
                past_part,
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        // let res: Verb = lookup("give");
        // println!("Give:\n{:#?}", res);
        // let res2: Verb = lookup("give up");
        // println!("Give: up\n{:#?}", res2);
        // let res3: Verb = lookup("take");
        // println!("Take:\n{:#?}", res3);
        // let res4: Verb = lookup("take off");
        // println!("Take off:\n{:#?}", res4);
        // let res5: Verb = lookup("have");
        // println!("Have:\n{:#?}", res5);
        // let res6: Verb = lookup("have at");
        // println!("Have at:\n{:#?}", res6);
        // let res7: Verb = lookup("get");
        // println!("Get:\n{:#?}", res7);
        // let res8: Verb = lookup("get in");
        // println!("Get in:\n{:#?}", res8);
        let res9: Verb = lookup("wug");
        println!("Wug:\n{:#?}", res9);
        let res10: Verb = lookup("wug up");
        println!("Wug up:\n{:#?}", res10);
        let res11: Verb = lookup("wuge");
        println!("Wuge:\n{:#?}", res11);
        let res12: Verb = lookup("wuge up");
        println!("Wuge up:\n{:#?}", res12);
    }
}
