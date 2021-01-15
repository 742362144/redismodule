use funcloc::func_loc_client::FuncLocClient;
use funcloc::InvokeRequest;

mod test;

pub mod funcloc {
    tonic::include_proto!("funcloc");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = FuncLocClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(InvokeRequest {
        request: "Tonic".into(),
    });

    let response = client.invoke(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
