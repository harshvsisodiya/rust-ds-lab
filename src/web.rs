use crate::{
    catalog::{self, Topic, TOPICS},
    escape, model,
};
use std::collections::BTreeMap;

fn decode(value: &str) -> Result<String, String> {
    let mut out = Vec::new();
    let bytes = value.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'+' => out.push(b' '),
            b'%' => {
                if i + 2 >= bytes.len() {
                    return Err("Incomplete URL encoding.".into());
                }
                let hex = std::str::from_utf8(&bytes[i + 1..i + 3])
                    .map_err(|_| "Invalid URL encoding.")?;
                out.push(u8::from_str_radix(hex, 16).map_err(|_| "Invalid URL encoding.")?);
                i += 2;
            }
            byte => out.push(byte),
        }
        i += 1;
    }
    String::from_utf8(out).map_err(|_| "Input must be UTF-8.".into())
}
fn encode(value: &str) -> String {
    value
        .bytes()
        .map(|b| {
            if b.is_ascii_alphanumeric() || b"-._~".contains(&b) {
                (b as char).to_string()
            } else {
                format!("%{b:02X}")
            }
        })
        .collect()
}
fn params(query: &str) -> Result<BTreeMap<String, String>, String> {
    if query.len() > 12000 {
        return Err("Input is too long.".into());
    }
    query
        .split('&')
        .filter(|s| !s.is_empty())
        .map(|part| {
            let (k, v) = part.split_once('=').unwrap_or((part, ""));
            Ok((decode(k)?, decode(v)?))
        })
        .collect()
}
fn hidden(name: &str, value: &str) -> String {
    format!(
        "<input type=\"hidden\" name=\"{}\" value=\"{}\">",
        escape(name),
        escape(value)
    )
}
fn url(topic: &str, data: &str, step: usize) -> String {
    format!(
        "/?topic={}&amp;data={}&amp;step={step}",
        encode(topic),
        encode(data)
    )
}

pub fn svg(d: &model::Diagram, title: &str) -> String {
    let mut s = format!(
        r##"<svg role="img" aria-labelledby="diagram-title" style="min-width:{}px" viewBox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg"><title id="diagram-title">{}</title><defs><marker id="arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse"><path d="M 0 0 L 10 5 L 0 10 z" fill="#7c91a6"/></marker></defs>"##,
        (d.width * 0.68).max(550.0),
        d.width,
        d.height,
        escape(title)
    );
    for edge in &d.edges {
        let a = &d.nodes[edge.from];
        let b = &d.nodes[edge.to];
        let end = if edge.directed {
            " marker-end=\"url(#arrow)\""
        } else {
            ""
        };
        let start = if edge.both {
            " marker-start=\"url(#arrow)\""
        } else {
            ""
        };
        if edge.from == edge.to {
            s.push_str(&format!(r###"<path d="M {} {} C {} {}, {} {}, {} {}" fill="none" stroke="#7c91a6" stroke-width="2"{end}/ >"###,a.x-35.0,a.y+20.0,a.x-100.0,a.y+100.0,a.x+100.0,a.y+100.0,a.x+35.0,a.y+20.0).replace("/ >","/>"));
        } else {
            let dx = b.x - a.x;
            let dy = b.y - a.y;
            // Intersect the ray with the 112x48 node box.
            let scale = 1.0 / (dx.abs() / 58.0).max(dy.abs() / 26.0).max(1.0);
            s.push_str(&format!(r##"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="#62788f" stroke-width="2"{end}{start}/>"##,a.x+dx*scale,a.y+dy*scale,b.x-dx*scale,b.y-dy*scale));
        }
    }
    for n in &d.nodes {
        let class = if n.active { "node active" } else { "node" };
        let font = if n.label.len() > 14 { 10 } else { 14 };
        s.push_str(&format!(r###"<g class="{class}"><title>{}: {}</title><rect x="{}" y="{}" width="112" height="48" rx="12"/><text x="{}" y="{}" text-anchor="middle" font-size="{font}">{}</text><text class="note" x="{}" y="{}" text-anchor="middle">{}</text></g>"###,escape(&n.label),escape(&n.note),n.x-56.0,n.y-24.0,n.x,n.y+5.0,escape(&n.label),n.x,n.y+45.0,escape(&n.note)));
    }
    if d.nodes.is_empty() {
        s.push_str(r###"<text x="500" y="120" text-anchor="middle" fill="#9caec0" font-size="20">Empty structure · insert an item or take the next step</text>"###);
    }
    s.push_str("</svg>");
    s
}

fn operations(topic: &Topic, data: &str, step: usize, len: usize) -> String {
    let id = topic.id;
    let label = match id {
        "queue" => "Enqueue",
        "stack" | "heap" => "Push",
        "deque" => "Push back",
        "graph" => "Add edge",
        "dsu" => "Union",
        _ => "Insert",
    };
    let placeholder = match id {
        "graph" | "dsu" => "A-B",
        "hashmap" => "cat:7",
        "hashset" => "cat",
        "trie" => "word",
        _ => "42",
    };
    let mut s=format!("<form class=\"operations\" method=\"get\" action=\"/\">{}{}{}<div class=\"fields\"><label>Value / key<input name=\"value\" placeholder=\"{placeholder}\" maxlength=\"32\"></label>",hidden("topic",id),hidden("data",data),hidden("step",&step.to_string()));
    if matches!(
        id,
        "array" | "singly" | "doubly" | "circular" | "fenwick" | "segment"
    ) {
        s.push_str(&format!("<label>Index (0-based)<input name=\"index\" type=\"number\" min=\"0\" max=\"{len}\" value=\"{len}\"></label>"));
    }
    s.push_str(&format!(
        "</div><div class=\"buttons\"><button name=\"op\" value=\"insert\">{label}</button>"
    ));
    if id == "deque" {
        s.push_str("<button name=\"op\" value=\"front\">Push front</button><button name=\"op\" value=\"popfront\">Pop front</button>");
    }
    if matches!(
        id,
        "array"
            | "stack"
            | "queue"
            | "deque"
            | "singly"
            | "doubly"
            | "circular"
            | "heap"
            | "hashset"
            | "hashmap"
            | "trie"
    ) {
        let label = match id {
            "queue" => "Dequeue",
            "stack" => "Pop",
            "deque" => "Pop back",
            "heap" => "Pop minimum",
            "hashmap" | "hashset" | "trie" => "Remove key/word",
            _ => "Remove at index",
        };
        s.push_str(&format!(
            "<button class=\"secondary\" name=\"op\" value=\"remove\">{label}</button>"
        ));
    }
    if matches!(id, "array" | "fenwick" | "segment") {
        s.push_str("<button class=\"secondary\" name=\"op\" value=\"set\">Set at index</button>");
    }
    s.push_str("</div></form>");
    s
}

pub fn page(query: &str) -> String {
    let (p, mut error) = match params(query) {
        Ok(p) => (p, String::new()),
        Err(e) => (BTreeMap::new(), e),
    };
    let get = |k: &str| p.get(k).map(String::as_str).unwrap_or("");
    let topic = catalog::topic(get("topic"));
    let grind = get("mode") == "grind";
    let mut data = p
        .get("data")
        .cloned()
        .unwrap_or_else(|| topic.example.into());
    let mut items = model::tokens(&data);
    if let Err(e) = model::validate(topic.id, &items) {
        error = e;
        items.clear();
    }
    let mut step = if get("step").is_empty() {
        items.len()
    } else {
        match get("step").parse::<usize>() {
            Ok(n) => n.min(items.len()),
            Err(_) => {
                error = "Step must be a nonnegative integer.".into();
                0
            }
        }
    };
    let mut message = String::new();
    if !get("op").is_empty() && error.is_empty() {
        let index = if get("index").is_empty() {
            Ok(step)
        } else {
            get("index").parse::<usize>()
        };
        match index {
            Err(_) => error = "Index must be a nonnegative integer.".into(),
            Ok(index) => {
                match model::mutate(topic.id, &items[..step], get("op"), get("value"), index) {
                    Ok((next, notice)) => {
                        items = next;
                        step = items.len();
                        data = items.join(",");
                        message = notice;
                    }
                    Err(e) => error = e,
                }
            }
        }
    }
    if message.is_empty() {
        message = if step == 0 {
            "Start with an empty structure. Next inserts the first input item.".into()
        } else {
            format!(
                "Step {step}: inserted {}. Each step shows the state after one complete insertion.",
                items[step - 1]
            )
        };
    }
    let mut s = format!(
        r###"<!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>{} · Rust DS Lab</title><style>{STYLE}</style></head><body><a class="skip" href="#main">Skip to content</a><div class="layout"><aside><a class="brand" href="/"><span class="logo">ds</span>Rust DS Lab</a><div class="eyebrow">THE STRUCTURE OF THINGS</div><nav aria-label="Data structures"><a class="grind-link" href="/?mode=grind">✦ Practice room <span>16</span></a>"###,
        if grind { "Practice room" } else { topic.title }
    );
    let mut group = "";
    for (i, t) in TOPICS.iter().enumerate() {
        if t.group != group {
            group = t.group;
            s.push_str(&format!("<h2 class=\"nav-group\">{}</h2>", escape(group)));
        }
        s.push_str(&format!(
            "<a {} href=\"/?topic={}\"><span class=\"nav-number\">{:02}</span>{}</a>",
            if t.id == topic.id && !grind {
                "aria-current=\"page\""
            } else {
                ""
            },
            t.id,
            i + 1,
            escape(t.title)
        ));
    }
    s.push_str("</nav><div class=\"aside-footer\"><span class=\"dot\"></span> Local · offline · Rust<br><small>No Python runtime required</small></div></aside><main id=\"main\"><header><span>LEARN BY BUILDING</span><span class=\"pill\">LINUX EDITION / 01</span></header>");
    if grind {
        s.push_str(&practice());
    } else {
        s.push_str(&format!("<div class=\"heading\"><div><div class=\"eyebrow\">{}</div><h1>{}</h1><p class=\"lead\">{}</p></div><span class=\"badge\">{}/16</span></div>",topic.group,topic.title,escape(topic.about),TOPICS.iter().position(|t|t.id==topic.id).unwrap()+1));
        s.push_str("<div class=\"workspace\"><div class=\"left-col\"><section class=\"card visual\"><div class=\"card-head\"><h2>Structure canvas</h2><span class=\"live\">● Interactive</span></div>");
        if !error.is_empty() {
            s.push_str(&format!(
                "<p class=\"error\" role=\"alert\">{}</p>",
                escape(&error)
            ));
        }
        let d = model::build(topic.id, &items[..step]);
        s.push_str(&format!(
            "<div class=\"canvas\">{}</div>",
            svg(&d, topic.title)
        ));
        if matches!(topic.id, "hashmap" | "hashset" | "trie") {
            s.push_str(
                "<p class=\"legend\">Mint marks stored entries / complete word endings.</p>",
            );
        } else {
            s.push_str("<p class=\"legend\">Labels show positions, ranges, or relationships. Use the table below for text details.</p>");
        }
        let prev = step.saturating_sub(1);
        let next = (step + 1).min(items.len());
        s.push_str(&format!("<div class=\"playback\"><a class=\"button secondary\" aria-label=\"Restart at empty\" href=\"{}\">↺ Restart</a><a class=\"button secondary\" href=\"{}\">← Previous</a><span>{step} / {}</span><a class=\"button\" href=\"{}\">Next →</a></div><p class=\"status\" role=\"status\">{}</p></section>",url(topic.id,&data,0),url(topic.id,&data,prev),items.len(),url(topic.id,&data,next),escape(&message)));
        s.push_str(&format!("<section class=\"card\"><div class=\"card-head\"><h2>Make it yours</h2><a href=\"/?topic={}\">Reset example</a></div><form method=\"get\" action=\"/\">{}<label>Input sequence <span>(comma-separated · max 16 items)</span><input name=\"data\" value=\"{}\" maxlength=\"512\"></label><button class=\"secondary\">Build sequence</button></form><h3>Try an operation</h3><p class=\"muted\">Operations use the displayed step and start a new replay sequence.</p>{}</section>",topic.id,hidden("topic",topic.id),escape(&data),operations(topic,&data,step,step)));
        s.push_str("<details class=\"card\"><summary>Inspect the structure as text</summary><table><caption>Visible nodes</caption><thead><tr><th>Value</th><th>Role</th></tr></thead><tbody>");
        for node in &d.nodes {
            s.push_str(&format!(
                "<tr><td>{}</td><td>{}</td></tr>",
                escape(&node.label),
                escape(&node.note)
            ));
        }
        if d.nodes.is_empty() {
            s.push_str("<tr><td colspan=\"2\">Empty structure</td></tr>");
        }
        s.push_str("</tbody></table>");
        if !d.rows.is_empty() {
            s.push_str("<table><caption>Internal representation</caption><tbody>");
            for (k, v) in &d.rows {
                s.push_str(&format!(
                    "<tr><th scope=\"row\">{}</th><td>{}</td></tr>",
                    escape(k),
                    escape(v)
                ));
            }
            s.push_str("</tbody></table>");
        }
        s.push_str("</details></div><div class=\"right-col\">");
        s.push_str(&format!("<section class=\"card code-card\"><div class=\"card-head\"><h2>Python equivalent</h2><span class=\"tag\">REFERENCE</span></div><p class=\"muted\">A standalone example of the same structure; it is not executed or synchronized with your inputs.</p><pre><code>{}</code></pre></section><section class=\"card concept\"><div class=\"eyebrow\">THE RULE THAT HOLDS</div><p>{}</p><h3>Time complexity</h3><p class=\"muted\">{}</p></section><section class=\"card challenge\"><div class=\"eyebrow\">YOUR TURN</div><h2>Predict before you click.</h2><p>{}</p><details><summary>Reveal explanation</summary><p>{}</p></details><a class=\"practice-cta\" href=\"/?mode=grind\">Enter the practice room →</a></section></div></div>",escape(topic.python),escape(topic.invariant),escape(topic.cost),escape(topic.challenge),escape(topic.answer)));
    }
    s.push_str("<footer>Built in Rust. Designed for curiosity.<span>16 core structures · no external crates</span></footer></main></div></body></html>");
    s
}

fn practice() -> String {
    let mut s=String::from("<div class=\"heading\"><div><div class=\"eyebrow\">PREDICT / CHECK / EXPLAIN</div><h1>Put your knowledge<br>to work.</h1><p class=\"lead\">One challenge per structure. Write your prediction, reveal the explanation, then test it in the lab.</p></div></div><section class=\"card routine\"><h2>A 20-minute practice loop</h2><p>5 min: rebuild yesterday’s example from memory. 10 min: predict three operation sequences. 5 min: explain the invariant and cost out loud.</p><p class=\"muted\">Self-check practice. Answers and progress are not saved; keep a notebook or your own practice log.</p></section><div class=\"practice-grid\">");
    for (i, t) in TOPICS.iter().enumerate() {
        s.push_str(&format!("<article class=\"card challenge\"><div class=\"eyebrow\">{:02} / {}</div><h2>{}</h2><p>{}</p><details><summary>Check your reasoning</summary><p>{}</p></details><a class=\"practice-cta\" href=\"/?topic={}\">Try it in the lab →</a></article>",i+1,t.group,t.title,escape(t.challenge),escape(t.answer),t.id));
    }
    s.push_str("</div>");
    s
}

const STYLE: &str = r#"
:root{color-scheme:dark;--bg:#0c121a;--panel:#141e2b;--border:#293646;--muted:#9bacc0;--mint:#a2edc4;--text:#eef3f8}*{box-sizing:border-box}body{margin:0;background:var(--bg);color:var(--text);font:15px/1.6 system-ui,-apple-system,BlinkMacSystemFont,'Segoe UI',sans-serif}a{color:var(--mint);text-decoration:none}a:hover{text-decoration:underline}button,input{font:inherit}button,a,input,summary{outline-offset:5px}:focus-visible{outline:3px solid var(--mint)}.skip{position:absolute;left:16px;top:-70px;z-index:5;background:var(--mint);color:#10271c;padding:12px}.skip:focus{top:12px}.layout{display:grid;grid-template-columns:244px 1fr;min-height:100vh}aside{background:#101923;border-right:1px solid var(--border);padding:30px 20px;position:sticky;top:0;height:100vh;overflow:auto}.brand{display:flex;align-items:center;gap:11px;color:#f0f6fa;font-size:20px;font-weight:700;letter-spacing:-.6px}.logo{background:var(--mint);color:#143427;border-radius:10px;padding:0 9px;font:700 25px/38px monospace}.eyebrow{font-size:10px;font-weight:750;letter-spacing:1.9px;color:var(--muted)}aside>.eyebrow{margin:20px 0 24px;font-size:8px;letter-spacing:1.5px}nav a{display:flex;align-items:center;gap:12px;padding:7px 10px;margin:3px 0;border-radius:7px;font-size:12px;color:#b5c3d2}nav a[aria-current]{background:#244437;color:var(--mint)}.nav-number{color:#70859a;font:10px monospace}.nav-group{font-size:9px;letter-spacing:1.8px;text-transform:uppercase;color:#74899e;margin:23px 10px 8px}.grind-link{border:1px solid #3c584c;color:var(--mint);padding:10px}.grind-link span{margin-left:auto;font:10px monospace}.aside-footer{margin-top:30px;font-size:11px;color:var(--muted)}.aside-footer small{font-size:9px}.dot{display:inline-block;width:6px;height:6px;background:var(--mint);border-radius:50%;margin-right:5px}main{padding:24px 42px 0;max-width:1800px;width:100%;min-width:0}header{display:flex;justify-content:space-between;align-items:center;color:#8da0b4;font-size:9px;letter-spacing:1.8px;border-bottom:1px solid var(--border);padding-bottom:23px}.pill{border:1px solid var(--border);padding:5px 10px;border-radius:20px;font-size:8px}.heading{display:flex;justify-content:space-between;align-items:center;margin:36px 0 28px;gap:20px}h1{font-size:clamp(30px,3.3vw,50px);letter-spacing:-1.9px;line-height:1.13;font-weight:650;margin:10px 0 15px}.lead{color:#acbace;max-width:720px;font-size:14px;line-height:1.8;margin:0}.badge{color:#74879c;font:12px monospace;border:1px solid var(--border);border-radius:50%;height:66px;width:66px;display:grid;place-items:center;flex-shrink:0}.workspace{display:grid;grid-template-columns:minmax(0,1.65fr) minmax(280px,1fr);gap:22px}.left-col,.right-col{min-width:0;display:flex;flex-direction:column;gap:20px}.card{border:1px solid var(--border);border-radius:13px;background:var(--panel);padding:22px;overflow:hidden}.card-head{display:flex;align-items:center;justify-content:space-between;gap:10px;margin-bottom:17px}.card-head h2{margin:0}.card-head>a{font-size:11px}h2{font-size:16px;letter-spacing:-.3px;margin:0 0 15px}h3{font-size:13px;margin:24px 0 8px}.live{font-size:10px;color:var(--mint)}.tag{font-size:8px;letter-spacing:1px;color:#a7bdd3;background:#213043;padding:4px 6px;border-radius:4px}.canvas{background-color:#101923;background-image:radial-gradient(#334151 .7px,transparent .7px);background-size:20px 20px;border:1px solid #263443;border-radius:10px;overflow:auto;min-height:240px;display:flex;align-items:center}.canvas svg{display:block;width:100%;min-width:550px;max-height:none}.node rect{fill:#1a2c3e;stroke:#5c7a97;stroke-width:1.5}.node text{fill:#e6f0fa;font-family:ui-monospace,monospace}.node.active rect{fill:#264a3c;stroke:#a2edc4}.node.active text{fill:#c1f9db}.node text.note{fill:#9aadc1;font-size:10px}.legend{font-size:10px;color:#9aadc1;margin:12px 0 18px}.playback{display:flex;gap:9px;align-items:center;flex-wrap:wrap}.playback>span{margin-left:auto;font:11px monospace;color:var(--muted)}button,.button{display:inline-block;background:var(--mint);border:1px solid var(--mint);color:#133325;padding:8px 13px;border-radius:6px;font-size:11px;font-weight:700;cursor:pointer}button:hover,.button:hover{filter:brightness(1.1);text-decoration:none}.secondary{background:#1c2a39;border-color:#35495c;color:#d5e2f0}.status{color:#b9ccdc;font-size:11px;padding-top:15px;border-top:1px solid var(--border);margin:18px 0 0}.error{color:#ffd8cd;background:#442721;border:1px solid #a46c58;padding:12px;border-radius:7px;font-size:12px}label{display:block;font-size:11px;color:#c8d5e2;margin:0 0 12px}label span{font-size:9px;color:var(--muted)}input{display:block;width:100%;background:#0e1721;border:1px solid #35465a;border-radius:6px;color:#e5eef8;padding:9px 11px;margin-top:7px;font:12px/1.6 ui-monospace,monospace}.fields{display:flex;gap:12px}.fields label{flex:1}.fields label:last-child:not(:first-child){max-width:145px}.buttons{display:flex;flex-wrap:wrap;gap:8px}.muted{color:var(--muted);font-size:11px;margin:8px 0 16px}.code-card{padding-bottom:12px}.code-card pre{overflow:auto;margin:0 -10px;padding:8px 10px 14px;tab-size:4;color:#bfdaca;font:11px/1.95 ui-monospace,SFMono-Regular,Consolas,monospace}.concept>p{font-size:14px}.concept>p.muted{font-size:12px}.challenge{background:linear-gradient(135deg,#1b3029,#16232c)}.challenge h2{margin-top:10px}.challenge p{font-size:12px;color:#c6d5df}details summary{cursor:pointer;color:var(--mint);font-size:12px;font-weight:600}details p{padding-top:8px}table{border-collapse:collapse;width:100%;font-size:12px;margin:15px 0}caption{text-align:left;color:var(--muted);margin:10px 0}th,td{text-align:left;border-bottom:1px solid var(--border);padding:8px;overflow-wrap:anywhere}.practice-cta{display:block;font-size:11px;font-weight:650;border-top:1px solid #395044;padding-top:13px;margin-top:20px}.practice-grid{display:grid;grid-template-columns:repeat(2,minmax(0,1fr));gap:20px}.routine{margin-bottom:25px}.routine p{font-size:13px}footer{display:flex;justify-content:space-between;gap:20px;border-top:1px solid var(--border);margin-top:38px;padding:22px 0;color:#778da3;font-size:10px}footer span{font-size:9px}@media(min-width:1600px){.canvas svg{min-width:650px}}@media(max-width:1150px){main{padding:22px}.layout{grid-template-columns:210px 1fr}aside{padding:24px 13px}.workspace{grid-template-columns:1fr}.right-col{display:grid;grid-template-columns:1fr 1fr;align-items:start}.code-card{grid-row:span 2}.canvas svg{min-width:550px}}@media(max-width:700px){.layout{display:block}aside{height:auto;position:static;border-right:0;border-bottom:1px solid var(--border);padding:16px}aside>.eyebrow,.nav-group,.aside-footer{display:none}nav{display:flex;overflow-x:auto;gap:6px;margin-top:14px}nav a{white-space:nowrap;padding:8px 10px}nav .grind-link{margin:3px 0}.nav-number{display:none}.brand{font-size:18px}main{padding:20px 16px}.heading{margin:26px 0}.badge{display:none}.right-col{display:flex}.practice-grid{grid-template-columns:1fr}.card{padding:17px}h1{font-size:34px}header{font-size:8px}.pill{font-size:7px}.playback{gap:6px}.playback .button{padding:7px 10px;font-size:10px}footer{flex-direction:column;gap:5px}.fields{flex-wrap:wrap}}
"#;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn url_encoding_round_trip() {
        for s in ["cat:7,dog:2", "a b&c", "हिन्दी", "<>\"'"] {
            assert_eq!(decode(&encode(s)).unwrap(), s);
        }
    }
    #[test]
    fn malformed_encoding_is_an_error() {
        assert!(decode("%ZZ").is_err());
        assert!(decode("%E").is_err());
        assert!(decode("%FF").is_err());
    }
    #[test]
    fn input_is_escaped() {
        let html = page("data=%3Cscript%3Ealert%281%29%3C%2Fscript%3E");
        assert!(!html.contains("<script>"));
        assert!(html.contains("&lt;script&gt;"));
    }
    #[test]
    fn every_topic_renders_and_steps() {
        for topic in TOPICS {
            for step in 0..=model::tokens(topic.example).len() {
                let html = page(&format!("topic={}&step={step}", topic.id));
                assert!(html.contains("</html>"));
                assert!(html.contains(topic.title));
                assert!(!html.contains("role=\"alert\""), "{}", topic.id);
            }
        }
    }
    #[test]
    fn invalid_input_does_not_mutate() {
        let html = page("topic=queue&data=oops&op=insert&value=4");
        assert!(html.contains("role=\"alert\""));
        assert!(!html.contains("Inserted 4"));
    }
    #[test]
    fn partial_replay_mutation_branches_from_visible_prefix() {
        let html = page("topic=stack&data=1,2,3&step=1&op=insert&value=9");
        assert!(html.contains("value=\"1,9\""));
        assert!(!html.contains("value=\"1,2,3,9\""));
    }
    #[test]
    fn practice_has_sixteen_challenges() {
        assert_eq!(
            page("mode=grind").matches("Check your reasoning").count(),
            16
        );
    }
}
