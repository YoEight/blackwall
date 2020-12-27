use blackwall_api::BWContext;

fn main() {
    let app = BWContext::new().unwrap().create_app(800, 600, "BlackWall");

    app.run()
}
