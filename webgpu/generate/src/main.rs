use std::{fs, path::Path};
use webidl2wit::{ConversionOptions, ResourceInheritance};

fn main() {
    let webidl = fs::read_to_string(Path::new("./webgpu-spec/spec/webgpu.idl")).unwrap();
    let webidl_ast = weedle::parse(&webidl).unwrap();
    let wit_ast = webidl2wit::webidl_to_wit(webidl_ast, ConversionOptions {
        phantom_interface: vec![
            "Navigator".into(),
            "WorkerNavigator".into(),
            "HTMLVideoElement".into(),
            "HTMLImageElement".into(),
            "VideoFrame".into(),
            "ImageBitmap".into(),
            "ImageData".into(),
            "EventTarget".into(),
            "DOMException".into(),
            "Event".into(),
        ],
        phantom_dictionaries: vec![
            "EventInit".into(),
            "PredefinedColorSpace".into(),
            "HTMLCanvasElement".into(),
            "OffscreenCanvas".into(),
        ],
        resource_inheritance: ResourceInheritance::DuplicateMethods,
        package_name: webidl2wit::PackageName::new("wasi", "webgpu", None),
        interface_name: "webgpu".into(),
        ..Default::default()
    },).unwrap();
    let wit = wit_ast.to_string();
    
    let mut resolve = wit_parser::Resolve::new();
    // TODO: remove once we have streams
    resolve.push_file(format!("../deps/io/poll.wit")).unwrap();

    resolve
        .push_str("", &wit)
        .unwrap();

    let mut packages = wit_encoder::packages_from_parsed(&resolve);

    // removes wasi:io/pollable
    // TODO: remove this once we get rid of pollable
    packages.remove(0);

    assert!(packages.len() == 1, "Should create exactly one package");
    let mut package = packages.remove(0);
    assert!(
        package.items().len() == 1,
        "Package should contain exactly one interface"
    );
    let item = package.items_mut().remove(0);
    let interface = match item {
        wit_encoder::PackageItem::Interface(interface) => interface,
        wit_encoder::PackageItem::World(_) => panic!("Package should contain exactly one interface"),
    };
    let transforms = serde_json::from_str(include_str!("../webgpu-transforms.json")).unwrap();
    let interface = wit_transforms::transform(interface, transforms);
    package.item(wit_encoder::PackageItem::Interface(interface));

    let output = package.to_string();

    fs::write("../webgpu.wit", output).unwrap();
}
