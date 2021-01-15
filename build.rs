fn main() {
    // tonic_build::configure()
    //     .type_attribute("routeguide.Point", "#[derive(Hash)]")
    //     .compile(&["proto/routeguide/route_guide.proto"], &["proto"])
    //     .unwrap();

    // tonic_build::compile_protos("proto/helloworld/helloworld.proto").unwrap();

    // see https://www.swiftdiaries.com/rust/tonic/
    tonic_build::compile_protos("proto/funcloc/funcloc.proto").unwrap();
    // tonic_build::compile_protos("proto/google/pubsub/pubsub.proto").unwrap();
}
