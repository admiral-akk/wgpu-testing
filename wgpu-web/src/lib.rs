extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{CanvasRenderingContext2d, Element, ImageData};
use wgpu_lib::{
    ray_tracing::structs::{sphere::Sphere, vec3::Vec3},
    structs::{color::Color, dimensions::Dimensions},
};

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

fn make_canvas(canvas: Element) -> Option<CanvasRenderingContext2d> {
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    return Some(
        canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap(),
    );
}

use wasm_bindgen::Clamped;

fn color(dimensions: &Dimensions, colors: &Vec<Color>) -> Result<ImageData, JsValue> {
    let mut u8Vec: Vec<u8> = Vec::new();
    for y in (0..dimensions.height).rev() {
        for x in 0..dimensions.width {
            let (r, g, b, a) = colors[dimensions.index(x, y)].to_rgba();
            u8Vec.push(r);
            u8Vec.push(g);
            u8Vec.push(b);
            u8Vec.push(a);
        }
    }
    ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&mut u8Vec),
        dimensions.width,
        dimensions.height,
    )
}

#[wasm_bindgen(start)]
pub async fn run() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let canvas = document.create_element("canvas")?;

    let dimensions = Dimensions::new(400, 400);
    let mut world: Vec<Sphere> = Vec::new();
    world.push(Sphere::new(Vec3::new(-1.0, 0.0, 15.0), 0.2));
    world.push(Sphere::new(Vec3::new(1.0, 0.0, 15.0), 0.2));
    world.push(Sphere::new(Vec3::new(0.5, 0.0, 5.0), 0.2));
    world.push(Sphere::new(Vec3::new(0.0, 0.0, 10.0), 1.0));
    world.push(Sphere::new(Vec3::new(3.0, 0.0, 20.0), 1.0));
    world.push(Sphere::new(Vec3::new(0.0, -1001.0, 10.0), 1000.0));

    let colors = wgpu_lib::get_ray_trace(&dimensions, world).await;
    body.append_child(&canvas)?;
    canvas.set_attribute("width", &dimensions.width.to_string())?;
    canvas.set_attribute("height", &dimensions.height.to_string())?;

    let render_context = make_canvas(canvas).unwrap();

    let image = color(&dimensions, &colors)?;
    render_context.put_image_data(&image, 0.0, 0.0)?;

    Ok(())
}
