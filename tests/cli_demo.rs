use std::process::Command;

#[test]
fn demo_cli_reports_offline_and_no_zone_violation() {
    let bin = env!("CARGO_BIN_EXE_project_blaze");
    let output = Command::new(bin)
        .output()
        .expect("failed to run demo binary");

    assert!(
        output.status.success(),
        "demo exited with non-zero status: {:?}",
        output.status
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("DEMO SUMMARY"),
        "demo summary missing from output"
    );

    let zone_line = stdout
        .lines()
        .find(|line| line.starts_with("zone_violation="))
        .expect("zone_violation line missing");
    assert_eq!(zone_line.trim(), "zone_violation=false");

    let offline_line = stdout
        .lines()
        .find(|line| line.starts_with("offline_robots="))
        .expect("offline_robots line missing");
    assert_ne!(offline_line.trim(), "offline_robots={}");
}
