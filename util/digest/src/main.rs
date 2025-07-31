use std::collections::HashMap;

use bakkie::{
    proto::V20250618::McpServer,
    provisions::{
        Provisions,
        tools::{Result, ToolError},
    },
};

use ashpd::desktop::{
    Icon,
    notification::{Button, Notification, NotificationProxy, Priority},
};

async fn run() -> ashpd::Result<()> {
    let proxy = NotificationProxy::new().await?;

    let notification_id = "org.gnome.design.Contrast";
    proxy
        .add_notification(
            notification_id,
            Notification::new("Contrast")
                .default_action("open")
                .default_action_target(100)
                .body("color copied to clipboard")
                .priority(Priority::High)
                .icon(Icon::with_names(&["dialog-question-symbolic"]))
                .button(Button::new("Copy", "copy").target(32))
                .button(Button::new("Delete", "delete").target(40)),
        )
        .await?;

    /*
    let action = proxy
        .receive_action_invoked()
        .await?
        .next()
        .await
        .expect("Stream exhausted");
    match action.name() {
        "copy" => (),   // Copy something to clipboard
        "delete" => (), // Delete the file
        _ => (),
    };
    println!("{:#?}", action.id());
    println!(
        "{:#?}",
        action.parameter().get(0).unwrap().downcast_ref::<u32>()
    );

    proxy.remove_notification(notification_id).await?;
    */
    Ok(())
}

/// Tabulates the characters that appear in the input for accurate counting.
#[bakkie::tool(title = "__count_letters")]
async fn count_letters(input: String) -> Result<HashMap<char, usize>> {
    let mut res = HashMap::new();

    for ch in input.chars() {
        *res.entry(ch).or_insert(0) += 1;
    }

    Ok(res)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    bakkie::dnp!();

    let provisions = Provisions::default();

    //provisions.insert_tool(greet()).await;
    //provisions.insert_tool(insert_into_db()).await;
    provisions.insert_tool(count_letters).await;

    let server = McpServer::new_with_provisions(bakkie::stdio(), provisions);
    server.run().await?;
    Ok(())
}
