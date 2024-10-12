use crate::em::execution_management::ExecutionManagement;
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
#[test]
fn testem(){
    let mut manager = ExecutionManagement::new();
    manager.load_manifest("/home/leehaonan/RustroverProjects/Buggy-Automotive/src/em/manifest.txt");
    manager.start_all();

    // 模拟执行管理器监视进程状态
    manager.monitor();

    println!("所有进程已终止。");
}