use wasm_bindgen::prelude::*;
use web_sys::{window, Document, HtmlCanvasElement, CanvasRenderingContext2d, Element};
use js_sys::Math;

#[wasm_bindgen]
pub struct ParticleAnimation {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    particles: Vec<Particle>,
}

struct Particle {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    radius: f64,
}

#[wasm_bindgen]
impl ParticleAnimation {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Result<ParticleAnimation, JsValue> {
        let window = window().ok_or_else(|| JsValue::from_str("No global window exists"))?;
        let document = window.document().ok_or_else(|| JsValue::from_str("No document exists"))?;
        let canvas = document
            .get_element_by_id(canvas_id)
            .ok_or_else(|| JsValue::from_str("Canvas element not found"))?
            .dyn_into::<HtmlCanvasElement>()?;

        let ctx = canvas
            .get_context("2d")?
            .ok_or_else(|| JsValue::from_str("Failed to get 2d context"))?
            .dyn_into::<CanvasRenderingContext2d>()?;

        let mut particles = Vec::new();
        for _ in 0..50 {
            particles.push(Particle {
                x: Math::random() * 400.0,
                y: Math::random() * 400.0,
                vx: (Math::random() - 0.5) * 1.5,
                vy: (Math::random() - 0.5) * 1.5,
                radius: Math::random() * 2.5 + 1.0,
            });
        }

        Ok(ParticleAnimation {
            canvas,
            ctx,
            particles,
        })
    }

    pub fn update(&mut self) {
        let width = 400.0;
        let height = 400.0;

        for p in &mut self.particles {
            p.x += p.vx;
            p.y += p.vy;

            if p.x < 0.0 || p.x > width {
                p.vx = -p.vx;
            }
            if p.y < 0.0 || p.y > height {
                p.vy = -p.vy;
            }
        }
    }

    pub fn draw(&self) {
        self.ctx.clear_rect(0.0, 0.0, 400.0, 400.0);
        self.ctx.set_fill_style(&JsValue::from_str("#f97316"));

        for p in &self.particles {
            self.ctx.begin_path();
            let _ = self.ctx.arc(p.x, p.y, p.radius, 0.0, std::f64::consts::PI * 2.0);
            self.ctx.fill();
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let window = window().ok_or_else(|| JsValue::from_str("No window"))?;
    let document = window.document().ok_or_else(|| JsValue::from_str("No document"))?;

    // Initialize Portfolio UI logic
    setup_navigation(&document)?;
    render_section(&document, "about")?;

    // Initialize Canvas Animation
    let anim = ParticleAnimation::new("particle-canvas")?;
    let anim_rc = std::rc::Rc::new(std::cell::RefCell::new(anim));
    let anim_clone = anim_rc.clone();

    let closure = closure::Closure::wrap(Box::new(move |_timestamp: f64| {
        let mut a = anim_clone.borrow_mut();
        a.update();
        a.draw();
        request_animation_frame(&closure_ref);
    }) as Box<dyn FnMut(f64)>);

    // Simplified loop runner for WASM
    Ok(())
}

fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) {
    window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register requestAnimationFrame callback");
}

// We need a stable reference mechanism for the animation loop
thread_local! {
    static GLOBAL_ANIM: std::cell::RefCell<Option<ParticleAnimation>> = std::cell::RefCell::new(None);
}

#[wasm_bindgen]
pub fn init_portfolio() -> Result<(), JsValue> {
    let window = window().ok_or_else(|| JsValue::from_str("No window"))?;
    let document = window.document().ok_or_else(|| JsValue::from_str("No document"))?;

    setup_navigation(&document)?;
    render_section(&document, "about")?;

    match ParticleAnimation::new("particle-canvas") {
        Ok(anim) => {
            GLOBAL_ANIM.with(|g| *g.borrow_mut() = Some(anim));
            start_loop();
        }
        Err(e) => web_sys::console::log_1(&e),
    }

    Ok();
    Ok(())
}

fn start_loop() {
    let f = std::rc::Rc::new(std::cell::RefCell::none());
    let f_clone = f.clone();
    *f_clone.borrow_mut() = Some(Closure::wrap(Box::new(move |timestamp: f64| {
        GLOBAL_ANIM.with(|g| {
            if let Some(anim) = g.borrow_mut().as_mut() {
                anim.update();
                anim.draw();
            }
        });
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut(f64)>));

    request_animation_frame(f.borrow().as_ref().unwrap());
}

use wasm_bindgen::closure::Closure;

fn setup_navigation(document: &Document) -> Result<(), JsValue> {
    let buttons = document.get_elements_by_class_name("nav-btn");
    for i in 0..buttons.length() {
        if let Some(btn) = buttons.item(i) {
            let doc_clone = document.clone();
            let btn_clone = btn.clone();
            let closure = Closure::wrap(Box::new(move |_e: web_sys::MouseEvent| {
                if let Some(section) = btn_clone.get_attribute("data-section") {
                    let _ = render_section(&doc_clone, &section);
                }
            }) as Box<dyn FnMut(web_sys::MouseEvent)>);
            btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
    }

    if let Some(explore) = document.get_element_by_id("explore-btn") {
        let doc_clone = document.clone();
        let closure = Closure::wrap(Box::new(move |_e: web_sys::MouseEvent| {
            let _ = render_section(&doc_clone, "projects");
        }) as Box<dyn FnMut(web_sys::MouseEvent)>);
        explore.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}

fn render_section(document: &Document, section: &str) -> Result<(), JsValue> {
    let container = document
        .get_element_by_id("content-section")
        .ok_or_else(|| JsValue::from_str("Content section not found"))?;

    let html_content = match section {
        "about" => r#"
            <h2 class="section-title">// 01. <span>About Me</span></h2>
            <div class="grid-container">
                <div class="card">
                    <h3>Background</h3>
                    <p>Systems engineer with over 6 years of experience building reliable backend services, concurrency primitives, and performant web applications.</p>
                    <div class="card-tags">
                        <span class="tag">Rust</span>
                        <span class="tag">C++</span>
                        <span class="tag">Distributed Systems</span>
                    </div>
                </div>
                <div class="card">
                    <h3>Philosophy</h3>
                    <p>I believe in zero-cost abstractions, strict type safety, and writing software that remains dependable under extreme workloads.</p>
                    <div class="card-tags">
                        <span class="tag">Memory Safety</span>
                        <span class="tag">Performance</span>
                        <span class="tag">Open Source</span>
                    </div>
                </div>
            </div>
        "#,
        "projects" => r#"
            <h2 class="section-title">// 02. <span>Featured Projects</span></h2>
            <div class="grid-container">
                <div class="card">
                    <h3>Tokio Hyper Gateway</h3>
                    <p>A lightning-fast reverse proxy and load balancer engineered in Rust, handling 100k+ concurrent requests with minimal memory footprint.</p>
                    <div class="card-tags">
                        <span class="tag">Rust</span>
                        <span class="tag">Tokio</span>
                        <span class="tag">Networking</span>
                    </div>
                </div>
                <div class="card">
                    <h3>WASM Canvas Engine</h3>
                    <p>A high-performance interactive graphics engine compiled directly to WebAssembly for smooth browser-based rendering.</p>
                    <div class="card-tags">
                        <span class="tag">WASM</span>
                        <span class="tag">Canvas API</span>
                        <span class="tag">Web-Sys</span>
                    </div>
                </div>
                <div class="card">
                    <h3>Distributed KV Store</h3>
                    <p>An append-only log structured key-value storage engine implementing the Raft consensus algorithm for distributed consistency.</p>
                    <div class="card-tags">
                        <span class="tag">Raft</span>
                        <span class="tag">Storage</span>
                        <span class="tag">Concurrency</span>
                    </div>
                </div>
            </div>
        "#,
        "skills" => r#"
            <h2 class="section-title">// 03. <span>Technical Expertise</span></h2>
            <div class="grid-container">
                <div class="card">
                    <h3>Systems & Languages</h3>
                    <p>Deep expertise in Rust memory management, async runtimes, unsafe blocks, and interoperability with C/C++ libraries.</p>
                    <div class="card-tags">
                        <span class="tag">Rust (Advanced)</span>
                        <span class="tag">Python</span>
                        <span class="tag">SQL</span>
                    </div>
                </div>
                <div class="card">
                    <h3>Web & Tooling</h3>
                    <p>Proficient in modern web technologies, WebAssembly bindings, Docker containerization, and automated CI/CD pipelines.</p>
                    <div class="card-tags">
                        <span class="tag">WASM</span>
                        <span class="tag">Docker</span>
                        <span class="tag">Git / CI</span>
                    </div>
                </div>
            </div>
        "#,
        "contact" => r#"
            <h2 class="section-title">// 04. <span>Get In Touch</span></h2>
            <div class="card" style="max-width: 600px;">
                <h3>Let's Build Something Together</h3>
                <p>Whether you have a question about systems architecture, rust consulting, or just want to say hi, my inbox is always open.</p>
                <div style="margin-top: 1.5rem; font-family: var(--font-mono); color: var(--accent-color);">
                    <p>Email: alex.rivers@system-architect.dev</p>
                    <p>GitHub: github.com/alex-rivers-rust</p>
                </div>
            </div>
        "#,
        _ => "<p>Section not found</p>",
    };

    container.set_inner_html(html_content);
    Ok(())
}
