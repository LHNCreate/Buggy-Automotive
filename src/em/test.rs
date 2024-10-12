
#[cfg(test)]
use super::execution_client::ExecutionClient;
#[test]
fn test1(){
    println!("begin to  execute test1");
    let event = ExecutionClient::termination(|| {println!("fuck dude")});
     if let Some(handler) = &event.termination_handler{
         handler();
     }
    event.termination_handler.as_ref().map(|handler|handler());

    assert!(event.termination_handler.is_some());
}