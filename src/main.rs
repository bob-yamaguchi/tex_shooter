use tex_shooter::Application;

static HTML_STR: &'static str = include_str!("../html/main.html");

fn main() {
    let mut app = Application::new();
/*
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    let _ = std::env::set_current_dir(&path);
    path.push("main.html");
    let html = std::fs::read_to_string(&path).unwrap();
*/
    let w_view = web_view::builder()
        .title("Tex Shooter")
        .content(web_view::Content::Html(/*html*/HTML_STR))
        .size(1280, 720)
        .user_data(())
        .invoke_handler(|webview, arg| {
            app.invoked(webview, arg);
            Ok(())
        })
        .build()
        .unwrap();
    let _ = w_view.run();
}

