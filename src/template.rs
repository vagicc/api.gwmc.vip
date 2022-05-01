use handlebars::{to_json, Handlebars};
use serde_json;
use serde_json::value::{Map, Value as Json};
/*
do_login_new中调用此处出错：
此处临时用string_to_static_str来解决，
应该按warp-0.3.1的examples中的examples/handlebars_template.rs示例来解决问题。
暂时先不处理，有空记得重新构造此处的函数
 */
/* 带广告的嵌入式页 */
pub fn to_html(name: &str, mut data: Map<String, Json>) -> String {
    let mut handlebars = Handlebars::new();

    /* 注册html模板 */
    handlebars
        .register_template_file(name, "src/views/".to_owned() + name)
        .unwrap_or_else(|e| println!("handlebars注册模板出错:{}", e));
    handlebars
        .register_template_file("frame.html", "src/views/frame.html")
        .unwrap_or_else(|e| println!("handlebars注册模板出错:{}", e));

    /* 传输数据给模板 */
    // let mut data = Map::new();
    data.insert("parent".to_string(), to_json("frame.html")); //必传,这个是插入父级的html
    data.insert("base_url".to_string(), to_json(crate::get_env("BASE_URL")));
    let html = handlebars.render(name, &data).unwrap();
    html
}

/* 带广告的嵌入式页 */
pub fn to_html_ad(name: &str, mut data: Map<String, Json>) -> String {
    let mut handlebars = Handlebars::new();

    /* 注册html模板 */
    handlebars
        .register_template_file(name, "src/views/".to_owned() + name)
        .unwrap_or_else(|e| println!("handlebars注册模板出错:{}", e));
    handlebars
        .register_template_file("frame.html", "src/views/frame.html")
        .unwrap_or_else(|e| println!("handlebars注册模板出错:{}", e));

    /* 传输数据给模板 */
    // let mut data = Map::new();
    data.insert("parent".to_string(), to_json("frame.html")); //必传,这个是插入父级的html
    data.insert("base_url".to_string(), to_json(crate::get_env("BASE_URL")));
    let html = handlebars.render(name, &data).unwrap();
    html
}

/* 基础版嵌入式页 */
pub fn to_html_base(name: &str, mut data: Map<String, Json>) -> String {
    let mut handlebars = Handlebars::new();

    /* 注册html模板 */
    handlebars
        .register_template_file(name, "src/views/".to_owned() + name)
        .unwrap_or_else(|e| println!("handlebars注册模板出错:{}", e));
    handlebars
        .register_template_file("frame_base.html", "src/views/frame_base.html")
        .unwrap_or_else(|e| println!("handlebars注册模板出错:{}", e));

    /* 传输数据给模板 */
    // let mut data = Map::new();
    data.insert("parent".to_string(), to_json("frame_base.html")); //必传,这个是插入父级的html
    data.insert("base_url".to_string(), to_json(crate::get_env("BASE_URL")));
    let html = handlebars.render(name, &data).unwrap();
    html
}

/* 单面 */
pub fn to_html_single(tpl_name: &str, mut data: Map<String, Json>) -> String {
    let mut handlebars = Handlebars::new();

    /* 注册html模板文件 */
    handlebars
        .register_template_file(tpl_name, "src/views/".to_owned() + tpl_name)
        .expect("handlebars注册模板出错");

    /* 传输数据给模板 */
    // let mut data = Map::new();
    data.insert("base_url".to_string(), to_json(crate::get_env("BASE_URL")));
    let html = handlebars.render(tpl_name, &data).expect("注册模板出错");
    html
}

/*
您可以这样做，但它涉及泄漏String的内存。
这不是你应该轻易做的事情。 通过泄漏String的内存，我们保证内存永远不会被释放（因此泄漏）。
因此，对内部对象的任何引用都可以解释为具有'static生命周期”。
*/
pub fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
