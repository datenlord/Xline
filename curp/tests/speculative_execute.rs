use curp::cmd::ProposeId;

use crate::common::{create_servers_client, TestCommand, TestCommandResult, TestCommandType};

mod common;

#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn speculative_execute() {
    let (mut rx, client) = create_servers_client().await;
    let result = client
        .propose(TestCommand::new(
            ProposeId::new("id1".to_owned()),
            TestCommandType::Get,
            vec!["A".to_owned()],
            None,
        ))
        .await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), TestCommandResult::GetResult("".to_owned()));

    let (t, key) = rx.recv().await.unwrap();
    assert_eq!(t, TestCommandType::Get);
    assert_eq!(key, "A".to_owned());
}
